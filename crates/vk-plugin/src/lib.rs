use std::collections::HashMap;
use std::fs;
use std::io::Read;

use wassel_sdk::bindings::{
    export,
    exports::wassel::foundation::http_handler::Guest,
    wasi::http::types::{IncomingRequest, ResponseOutparam},
    wasi::io::streams::StreamError,
    wassel::foundation::postgres::{self, Parameter},
};
use wassel_sdk::http::{IntoResponse, client};

use bot_core::{BotHandler, db, menu::MenuNode, messenger::BotResponse, schedule, texts::Texts};
use vk_api::{callback::CallbackEvent, photo, sender::VkSender};

struct Plugin;

fn get_config(key: &str) -> String {
    wassel_sdk::bindings::wasi_config::store::get(key)
        .unwrap_or_default()
        .unwrap_or_default()
}

impl Guest for Plugin {
    fn handle_request(request: IncomingRequest, response_out: ResponseOutparam) {
        let db_connection = get_config("db_connection");
        let vk_token = get_config("vk_token");
        let vk_confirmation_code = get_config("vk_confirmation_code");
        let deanery_host = get_config("deanery_host");

        run_migrations(&db_connection);

        let body = match read_request_body(&request) {
            Ok(b) => b,
            Err(_) => {
                "Bad request"
                    .into_response()
                    .write_to_response_outparam(response_out);
                return;
            }
        };

        let event = match CallbackEvent::try_from(body.as_slice()) {
            Ok(e) => e,
            Err(_) => {
                "Bad request"
                    .into_response()
                    .write_to_response_outparam(response_out);
                return;
            }
        };

        match event.event_type.as_str() {
            "confirmation" => {
                vk_confirmation_code
                    .into_response()
                    .write_to_response_outparam(response_out);
                return;
            }
            "message_new" => {
                if let Some(message) = event.into_message() {
                    handle_message_new(message, &db_connection, &vk_token, &deanery_host);
                }
            }
            _ => {}
        }

        "ok".into_response()
            .write_to_response_outparam(response_out);
    }
}

fn run_migrations(db_connection: &str) {
    let config = postgres::ConnectionConfig::new(db_connection);
    let conn = match postgres::Connection::open(config) {
        Ok(c) => c,
        Err(_) => return,
    };
    let _ = conn.execute(db::CREATE_TABLES, &[]);
}

fn handle_message_new(
    message: vk_api::callback::Message,
    db_connection: &str,
    vk_token: &str,
    deanery_host: &str,
) {
    let peer_id = message.peer_id;

    let config = postgres::ConnectionConfig::new(db_connection);
    let conn = match postgres::Connection::open(config) {
        Ok(c) => c,
        Err(_) => return,
    };

    let (user_id, current_node_id, student_group) = get_or_create_user(&conn, "vk", peer_id);
    let menu_nodes = load_menu_nodes(&conn);
    let texts = load_texts(&conn);
    let handler = BotHandler::new(menu_nodes, texts);

    let payload = message.payload.as_deref().map(|p| p.trim_matches('"'));

    let is_in_schedule = current_node_id
        .and_then(|nid| handler.menu_nodes_ref().iter().find(|n| n.id == nid))
        .map(|n| n.slug == "schedule")
        .unwrap_or(false);

    let is_group_input = is_in_schedule
        && payload.is_none()
        && !message.text.is_empty()
        && !message.text.starts_with('/');

    if is_group_input {
        let group = message.text.trim().to_uppercase();

        let check_url = format!(
            "{}/api/schedule/lessons?group={}&weekday=Monday&parity=Odd",
            deanery_host,
            schedule::urlencode(&group)
        );
        let group_exists = match client::get(check_url).send() {
            Ok(resp) => {
                let bytes = read_body(resp.into_body());
                match serde_json::from_slice::<Vec<schedule::Lesson>>(&bytes) {
                    Ok(lessons) => !lessons.is_empty(),
                    Err(_) => false,
                }
            }
            Err(_) => false,
        };

        if group_exists {
            let _ = conn.execute(
                db::UPDATE_USER_GROUP,
                &[Parameter::Text(group.clone()), Parameter::Int64(user_id)],
            );
            let response = handler.handle_message(current_node_id, None, None, Some(&group));
            process_response(
                response,
                peer_id,
                &conn,
                user_id,
                vk_token,
                deanery_host,
                handler.texts_ref(),
            );
        } else {
            let sender = VkSender::new(vk_token.to_string());
            let buttons = vec![
                bot_core::messenger::Button {
                    label: handler.texts_ref().get("btn.back").into(),
                    payload: "schedule_back".into(),
                },
                bot_core::messenger::Button {
                    label: handler.texts_ref().get("btn.home").into(),
                    payload: "1".into(),
                },
            ];
            let url = sender.build_send_url(
                peer_id,
                handler.texts_ref().get("msg.schedule_invalid_group"),
                &buttons,
                None,
            );
            let _ = client::get(url).send();
        }
        return;
    }

    let response = handler.handle_message(
        current_node_id,
        payload,
        Some(&message.text),
        student_group.as_deref(),
    );
    process_response(
        response,
        peer_id,
        &conn,
        user_id,
        vk_token,
        deanery_host,
        handler.texts_ref(),
    );
}

fn process_response(
    response: BotResponse,
    peer_id: i64,
    conn: &postgres::Connection,
    user_id: i64,
    vk_token: &str,
    deanery_host: &str,
    texts: &Texts,
) {
    let sender = VkSender::new(vk_token.to_string());

    match response {
        BotResponse::Message(msg, new_node_id) => {
            if let Some(node_id) = new_node_id {
                let _ = conn.execute(
                    db::UPDATE_USER_NODE,
                    &[Parameter::Int64(node_id), Parameter::Int64(user_id)],
                );
            }
            let attachment = msg
                .image_url
                .as_deref()
                .and_then(|path| upload_photo(&sender, peer_id, path));
            let url =
                sender.build_send_url(peer_id, &msg.text, &msg.buttons, attachment.as_deref());
            let _ = client::get(url).send();
        }
        BotResponse::ScheduleRequest {
            group,
            weekday,
            parity,
            new_node_id,
        } => {
            if let Some(node_id) = new_node_id {
                let _ = conn.execute(
                    db::UPDATE_USER_NODE,
                    &[Parameter::Int64(node_id), Parameter::Int64(user_id)],
                );
            }

            let url = format!(
                "{}/api/schedule/lessons?group={}&weekday={}&parity={}",
                deanery_host,
                schedule::urlencode(&group),
                weekday,
                parity
            );
            let text = match client::get(url).send() {
                Ok(resp) => {
                    let bytes = read_body(resp.into_body());
                    match serde_json::from_slice::<Vec<schedule::Lesson>>(&bytes) {
                        Ok(lessons) if lessons.is_empty() => {
                            texts.get("msg.schedule_day_off").to_string()
                        }
                        Ok(lessons) => schedule::format_lessons(&lessons, &weekday, &parity),
                        Err(_) => texts.get("msg.schedule_error").to_string(),
                    }
                }
                Err(_) => texts.get("msg.schedule_connection_error").to_string(),
            };

            let buttons = schedule_buttons(texts);
            let url = sender.build_send_url(peer_id, &text, &buttons, None);
            let _ = client::get(url).send();
        }
        BotResponse::ScheduleWeekRequest {
            group,
            parity,
            new_node_id,
        } => {
            if let Some(node_id) = new_node_id {
                let _ = conn.execute(
                    db::UPDATE_USER_NODE,
                    &[Parameter::Int64(node_id), Parameter::Int64(user_id)],
                );
            }

            let mut full_text = String::new();
            for weekday in schedule::WEEKDAYS {
                let url = format!(
                    "{}/api/schedule/lessons?group={}&weekday={}&parity={}",
                    deanery_host,
                    schedule::urlencode(&group),
                    weekday,
                    parity
                );
                if let Ok(resp) = client::get(url).send() {
                    let bytes = read_body(resp.into_body());
                    if let Ok(lessons) = serde_json::from_slice::<Vec<schedule::Lesson>>(&bytes) {
                        full_text.push_str(&schedule::format_lessons(&lessons, weekday, &parity));
                        full_text.push('\n');
                    }
                }
            }

            if full_text.is_empty() {
                full_text = texts.get("msg.schedule_week_error").to_string();
            }

            let buttons = schedule_buttons(texts);
            let url = sender.build_send_url(peer_id, &full_text, &buttons, None);
            let _ = client::get(url).send();
        }
        BotResponse::AskGroup { new_node_id } => {
            if let Some(node_id) = new_node_id {
                let _ = conn.execute(
                    db::UPDATE_USER_NODE,
                    &[Parameter::Int64(node_id), Parameter::Int64(user_id)],
                );
            }
            let _ = conn.execute(db::CLEAR_USER_GROUP, &[Parameter::Int64(user_id)]);

            let buttons = vec![
                bot_core::messenger::Button {
                    label: texts.get("btn.back").into(),
                    payload: "schedule_back".into(),
                },
                bot_core::messenger::Button {
                    label: texts.get("btn.home").into(),
                    payload: "1".into(),
                },
            ];
            let url = sender.build_send_url(peer_id, texts.get("msg.ask_group"), &buttons, None);
            let _ = client::get(url).send();
        }
    }
}

fn schedule_buttons(texts: &Texts) -> Vec<bot_core::messenger::Button> {
    vec![
        bot_core::messenger::Button {
            label: texts.get("btn.schedule_today").into(),
            payload: "schedule_today".into(),
        },
        bot_core::messenger::Button {
            label: texts.get("btn.schedule_tomorrow").into(),
            payload: "schedule_tomorrow".into(),
        },
        bot_core::messenger::Button {
            label: texts.get("btn.schedule_this_week").into(),
            payload: "schedule_this_week".into(),
        },
        bot_core::messenger::Button {
            label: texts.get("btn.schedule_next_week").into(),
            payload: "schedule_next_week".into(),
        },
        bot_core::messenger::Button {
            label: texts.get("btn.schedule_change_group").into(),
            payload: "schedule_change_group".into(),
        },
        bot_core::messenger::Button {
            label: texts.get("btn.back").into(),
            payload: "schedule_back".into(),
        },
        bot_core::messenger::Button {
            label: texts.get("btn.home").into(),
            payload: "1".into(),
        },
    ]
}

fn upload_photo(sender: &VkSender, peer_id: i64, file_path: &str) -> Option<String> {
    let file_bytes = fs::read(file_path).ok()?;
    let (content_type, multipart_body) = photo::build_multipart_body(&file_bytes, "photo.jpg");

    let url = sender.build_get_upload_server_url(peer_id);
    let resp = client::get(url).send().ok()?;
    let resp_body = read_body(resp.into_body());
    let upload_server: photo::UploadServerResponse = serde_json::from_slice(&resp_body).ok()?;
    let upload_url = upload_server.response?.upload_url;

    let resp = client::post(upload_url)
        .header(
            "content-type",
            content_type.parse::<http::HeaderValue>().unwrap(),
        )
        .body(multipart_body)
        .send()
        .ok()?;
    let resp_body = read_body(resp.into_body());
    let upload_result: photo::UploadResult = serde_json::from_slice(&resp_body).ok()?;

    let url = sender.build_save_photo_url(
        &upload_result.photo,
        upload_result.server,
        &upload_result.hash,
    );
    let resp = client::get(url).send().ok()?;
    let resp_body = read_body(resp.into_body());
    let save_resp: photo::SavePhotoResponse = serde_json::from_slice(&resp_body).ok()?;
    let saved = save_resp.response?.into_iter().next()?;

    Some(saved.to_attachment())
}

fn read_body(mut body: wassel_sdk::http::Body) -> Vec<u8> {
    let mut buf = Vec::new();
    let _ = body.read_to_end(&mut buf);
    buf
}

fn get_or_create_user(
    conn: &postgres::Connection,
    platform: &str,
    platform_user_id: i64,
) -> (i64, Option<i64>, Option<String>) {
    if let Ok(rows) = conn.query(
        db::SELECT_USER,
        &[
            Parameter::Text(platform.to_string()),
            Parameter::Int64(platform_user_id),
        ],
    ) && let Some(row) = rows.rows.first()
    {
        let user_id = match &row[0] {
            postgres::Value::Int64(id) => *id,
            _ => return (0, None, None),
        };
        let node_id = match &row[1] {
            postgres::Value::Int64(id) => Some(*id),
            _ => None,
        };
        let group = match &row[2] {
            postgres::Value::Text(g) => Some(g.clone()),
            _ => None,
        };
        return (user_id, node_id, group);
    }
    if let Ok(rows) = conn.query(
        db::UPSERT_USER,
        &[
            Parameter::Text(platform.to_string()),
            Parameter::Int64(platform_user_id),
        ],
    ) && let Some(row) = rows.rows.first()
    {
        let user_id = match &row[0] {
            postgres::Value::Int64(id) => *id,
            _ => 0,
        };
        return (user_id, None, None);
    }
    (0, None, None)
}

fn load_menu_nodes(conn: &postgres::Connection) -> Vec<MenuNode> {
    let rows = match conn.query(db::SELECT_ALL_MENU_NODES, &[]) {
        Ok(r) => r,
        Err(_) => return vec![],
    };
    rows.rows
        .iter()
        .filter_map(|row| {
            Some(MenuNode {
                id: match &row[0] {
                    postgres::Value::Int64(v) => *v,
                    _ => return None,
                },
                parent_id: match &row[1] {
                    postgres::Value::Int64(v) => Some(*v),
                    _ => None,
                },
                slug: match &row[2] {
                    postgres::Value::Text(v) => v.clone(),
                    _ => return None,
                },
                title: match &row[3] {
                    postgres::Value::Text(v) => v.clone(),
                    _ => return None,
                },
                content: match &row[4] {
                    postgres::Value::Text(v) => Some(v.clone()),
                    _ => None,
                },
                image_url: match &row[5] {
                    postgres::Value::Text(v) => Some(v.clone()),
                    _ => None,
                },
                sort_order: match &row[6] {
                    postgres::Value::Int32(v) => *v,
                    _ => 0,
                },
            })
        })
        .collect()
}

fn load_texts(conn: &postgres::Connection) -> Texts {
    let mut map = HashMap::new();
    if let Ok(rows) = conn.query(db::SELECT_ALL_TEXTS, &[]) {
        for row in &rows.rows {
            if let (Some(postgres::Value::Text(key)), Some(postgres::Value::Text(value))) =
                (row.first(), row.get(1))
            {
                map.insert(key.clone(), value.clone());
            }
        }
    }
    Texts::new(map)
}

fn read_request_body(request: &IncomingRequest) -> Result<Vec<u8>, String> {
    let body = request.consume().map_err(|_| "No body")?;
    let stream = body.stream().map_err(|_| "No stream")?;
    let mut buf = Vec::new();
    loop {
        match stream.blocking_read(4096) {
            Err(StreamError::Closed) => break,
            Err(e) => return Err(format!("{e:?}")),
            Ok(vec) => {
                if vec.is_empty() {
                    break;
                }
                buf.extend_from_slice(&vec);
            }
        }
    }
    Ok(buf)
}

export!(Plugin);
