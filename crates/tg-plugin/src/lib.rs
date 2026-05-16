use std::collections::HashMap;
use std::fs;
use std::io::Read;

use wassel_sdk::bindings::wassel::foundation::postgres::{self, Parameter};
use wassel_sdk::http::{IntoResponse, Request, Response, client, handler};

use bot_core::{BotHandler, db, menu::MenuNode, messenger::BotResponse, schedule, texts::Texts};
use tg_api::{sender::TgSender, webhook::Update};

fn get_config(key: &str) -> String {
    wassel_sdk::bindings::wasi_config::store::get(key)
        .unwrap_or_default()
        .unwrap_or_default()
}

#[handler]
fn handle_request(request: Request) -> Response {
    let db_connection = get_config("db_connection");
    let tg_bot_token = get_config("tg_bot_token");
    let deanery_host = get_config("deanery_host");

    run_migrations(&db_connection);

    let mut body = request.into_body();
    let mut buf = Vec::new();
    let _ = body.read_to_end(&mut buf);

    let update = match Update::parse(&buf) {
        Ok(u) => u,
        Err(_) => return "Bad request".into_response(),
    };

    handle_update(update, &db_connection, &tg_bot_token, &deanery_host);

    "ok".into_response()
}

fn run_migrations(db_connection: &str) {
    let config = postgres::ConnectionConfig::new(db_connection);
    let conn = match postgres::Connection::open(config) {
        Ok(c) => c,
        Err(_) => return,
    };
    let _ = conn.execute(db::CREATE_TABLES, &[]);
}

fn handle_update(update: Update, db_connection: &str, tg_bot_token: &str, deanery_host: &str) {
    let (chat_id, payload, text, callback_query_id) = match update.extract() {
        Some(data) => data,
        None => return,
    };

    let config = postgres::ConnectionConfig::new(db_connection);
    let conn = match postgres::Connection::open(config) {
        Ok(c) => c,
        Err(_) => return,
    };

    let (user_id, current_node_id, student_group) = get_or_create_user(&conn, "tg", chat_id);
    let menu_nodes = load_menu_nodes(&conn);
    let texts = load_texts(&conn);
    let handler = BotHandler::new(menu_nodes, texts);

    if let Some(cb_id) = callback_query_id {
        let sender = TgSender::new(tg_bot_token.to_string());
        let url = sender.build_answer_callback_url(cb_id);
        let _ = client::get(url).send();
    }

    let is_in_schedule = current_node_id
        .and_then(|nid| handler.menu_nodes_ref().iter().find(|n| n.id == nid))
        .map(|n| n.slug == "schedule")
        .unwrap_or(false);

    let text_str = text.unwrap_or("");
    let is_group_input =
        is_in_schedule && payload.is_none() && !text_str.is_empty() && !text_str.starts_with('/');

    if is_group_input {
        let group = text_str.trim().to_uppercase();

        let check_url = format!(
            "{}/api/schedule/lessons?group={}&weekday=Monday&parity=Odd",
            deanery_host,
            schedule::urlencode(&group)
        );
        let group_exists = match client::get(check_url).send() {
            Ok(resp) => {
                let body = resp.into_body();
                let bytes = read_body(body);
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
                chat_id,
                &conn,
                user_id,
                tg_bot_token,
                deanery_host,
                handler.texts_ref(),
            );
        } else {
            let sender = TgSender::new(tg_bot_token.to_string());
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
                chat_id,
                handler.texts_ref().get("msg.schedule_invalid_group"),
                &buttons,
            );
            let _ = client::get(url).send();
        }
        return;
    }

    let response = handler.handle_message(current_node_id, payload, text, student_group.as_deref());
    process_response(
        response,
        chat_id,
        &conn,
        user_id,
        tg_bot_token,
        deanery_host,
        handler.texts_ref(),
    );
}

fn process_response(
    response: BotResponse,
    chat_id: i64,
    conn: &postgres::Connection,
    user_id: i64,
    tg_bot_token: &str,
    deanery_host: &str,
    texts: &Texts,
) {
    let sender = TgSender::new(tg_bot_token.to_string());

    match response {
        BotResponse::Message(msg, new_node_id) => {
            if let Some(node_id) = new_node_id {
                let _ = conn.execute(
                    db::UPDATE_USER_NODE,
                    &[Parameter::Int64(node_id), Parameter::Int64(user_id)],
                );
            }

            if let Some(image_path) = msg.image_url.as_deref() {
                if let Ok(photo_bytes) = fs::read(image_path) {
                    let (content_type, multipart_body) = sender.build_send_photo_body(
                        chat_id,
                        &photo_bytes,
                        &msg.text,
                        &msg.buttons,
                    );
                    let _ = client::post(sender.send_photo_url())
                        .header(
                            "content-type",
                            content_type.parse::<http::HeaderValue>().unwrap(),
                        )
                        .body(multipart_body)
                        .send();
                }
            } else {
                let url = sender.build_send_url(chat_id, &msg.text, &msg.buttons);
                let _ = client::get(url).send();
            }
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
            let url = sender.build_send_url(chat_id, &text, &buttons);
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
            let url = sender.build_send_url(chat_id, &full_text, &buttons);
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
            let url = sender.build_send_url(chat_id, texts.get("msg.ask_group"), &buttons);
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
