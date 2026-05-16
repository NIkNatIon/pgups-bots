use std::fs;
use std::collections::HashMap;

use wassel_sdk::bindings::{
    export,
    exports::wassel::foundation::http_handler::Guest,
    wasi::http::types::{
        Fields, IncomingRequest, IncomingResponse, Method, OutgoingBody, OutgoingRequest,
        OutgoingResponse, ResponseOutparam,
    },
    wasi::io::streams::StreamError,
    wassel::foundation::{http_client, postgres::{self, Parameter}},
};

use bot_core::{BotHandler, db, menu::MenuNode, messenger::BotResponse, schedule, texts::Texts};
use tg_api::{sender::TgSender, webhook::Update};

struct Plugin;

fn get_config(key: &str) -> String {
    wassel_sdk::bindings::wasi_config::store::get(key)
        .unwrap_or_default()
        .unwrap_or_default()
}

impl Guest for Plugin {
    fn handle_request(request: IncomingRequest, response_out: ResponseOutparam) {
        let db_connection = get_config("db_connection");
        let tg_bot_token = get_config("tg_bot_token");
        let deanery_host = get_config("deanery_host");

        run_migrations(&db_connection);

        let body = match read_request_body(&request) {
            Ok(b) => b,
            Err(_) => {
                write_response(response_out, 400, b"Bad request");
                return;
            }
        };

        let update = match Update::parse(&body) {
            Ok(u) => u,
            Err(_) => {
                write_response(response_out, 400, b"Invalid JSON");
                return;
            }
        };

        handle_update(update, &db_connection, &tg_bot_token, &deanery_host);

        write_response(response_out, 200, b"ok");
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
        let req = OutgoingRequest::new(Fields::new());
        let _ = http_client::send(&url, req);
    }

    let is_in_schedule = current_node_id
        .and_then(|nid| handler.menu_nodes_ref().iter().find(|n| n.id == nid))
        .map(|n| n.slug == "schedule")
        .unwrap_or(false);

    let text_str = text.unwrap_or("");
    let is_group_input = is_in_schedule && payload.is_none() && !text_str.is_empty() && !text_str.starts_with('/');

    if is_group_input {
        let group = text_str.trim().to_uppercase();

        let check_url = format!(
            "{}/api/schedule/lessons?group={}&weekday=Monday&parity=Odd",
            deanery_host, schedule::urlencode(&group)
        );
        let req = OutgoingRequest::new(Fields::new());
        let group_exists = match http_client::send(&check_url, req) {
            Ok(resp) => match read_response_body(&resp) {
                Some(body) => match serde_json::from_slice::<Vec<schedule::Lesson>>(&body) {
                    Ok(lessons) => !lessons.is_empty(),
                    Err(_) => false,
                },
                None => false,
            },
            Err(_) => false,
        };

        if group_exists {
            let _ = conn.execute(
                db::UPDATE_USER_GROUP,
                &[Parameter::Text(group.clone()), Parameter::Int64(user_id)],
            );
            let response = handler.handle_message(current_node_id, None, None, Some(&group));
            process_response(response, chat_id, &conn, user_id, tg_bot_token, deanery_host, handler.texts_ref());
        } else {
            let sender = TgSender::new(tg_bot_token.to_string());
            let buttons = vec![
                bot_core::messenger::Button { label: handler.texts_ref().get("btn.back").into(), payload: "schedule_back".into() },
                bot_core::messenger::Button { label: handler.texts_ref().get("btn.home").into(), payload: "1".into() },
            ];
            let url = sender.build_send_url(chat_id, handler.texts_ref().get("msg.schedule_invalid_group"), &buttons);
            let req = OutgoingRequest::new(Fields::new());
            let _ = http_client::send(&url, req);
        }
        return;
    }

    let response = handler.handle_message(current_node_id, payload, text, student_group.as_deref());
    process_response(response, chat_id, &conn, user_id, tg_bot_token, deanery_host, handler.texts_ref());
}

fn process_response(response: BotResponse, chat_id: i64, conn: &postgres::Connection, user_id: i64, tg_bot_token: &str, deanery_host: &str, texts: &Texts) {
    let sender = TgSender::new(tg_bot_token.to_string());

    match response {
        BotResponse::Message(msg, new_node_id) => {
            if let Some(node_id) = new_node_id {
                let _ = conn.execute(db::UPDATE_USER_NODE, &[Parameter::Int64(node_id), Parameter::Int64(user_id)]);
            }

            if let Some(image_path) = msg.image_url.as_deref() {
                if let Ok(photo_bytes) = fs::read(image_path) {
                    let (content_type, multipart_body) = sender.build_send_photo_body(
                        chat_id, &photo_bytes, &msg.text, &msg.buttons,
                    );
                    let headers = Fields::new();
                    headers.set(&"content-type".to_string(), &[content_type.as_bytes().to_vec()]).unwrap();
                    let req = OutgoingRequest::new(headers);
                    req.set_method(&Method::Post).unwrap();
                    let req_body = req.body().unwrap();
                    { let stream = req_body.write().unwrap(); stream.write(&multipart_body).unwrap(); }
                    OutgoingBody::finish(req_body, None).unwrap();
                    let _ = http_client::send(&sender.send_photo_url(), req);
                }
            } else {
                let url = sender.build_send_url(chat_id, &msg.text, &msg.buttons);
                let req = OutgoingRequest::new(Fields::new());
                let _ = http_client::send(&url, req);
            }
        }
        BotResponse::ScheduleRequest { group, weekday, parity, new_node_id } => {
            if let Some(node_id) = new_node_id {
                let _ = conn.execute(db::UPDATE_USER_NODE, &[Parameter::Int64(node_id), Parameter::Int64(user_id)]);
            }

            let url = format!(
                "{}/api/schedule/lessons?group={}&weekday={}&parity={}",
                deanery_host, schedule::urlencode(&group), weekday, parity
            );
            let req = OutgoingRequest::new(Fields::new());
            let text = match http_client::send(&url, req) {
                Ok(resp) => match read_response_body(&resp) {
                    Some(body) => match serde_json::from_slice::<Vec<schedule::Lesson>>(&body) {
                        Ok(lessons) if lessons.is_empty() => texts.get("msg.schedule_invalid_group").to_string(),
                        Ok(lessons) => schedule::format_lessons(&lessons, &weekday, &parity),
                        Err(_) => texts.get("msg.schedule_error").to_string(),
                    },
                    None => texts.get("msg.schedule_server_down").to_string(),
                },
                Err(_) => texts.get("msg.schedule_connection_error").to_string(),
            };

            let buttons = schedule_buttons(texts);
            let url = sender.build_send_url(chat_id, &text, &buttons);
            let req = OutgoingRequest::new(Fields::new());
            let _ = http_client::send(&url, req);
        }
        BotResponse::ScheduleWeekRequest { group, parity, new_node_id } => {
            if let Some(node_id) = new_node_id {
                let _ = conn.execute(db::UPDATE_USER_NODE, &[Parameter::Int64(node_id), Parameter::Int64(user_id)]);
            }

            let mut full_text = String::new();
            for weekday in schedule::WEEKDAYS {
                let url = format!(
                    "{}/api/schedule/lessons?group={}&weekday={}&parity={}",
                    deanery_host, schedule::urlencode(&group), weekday, parity
                );
                let req = OutgoingRequest::new(Fields::new());
                if let Ok(resp) = http_client::send(&url, req) {
                    if let Some(body) = read_response_body(&resp) {
                        if let Ok(lessons) = serde_json::from_slice::<Vec<schedule::Lesson>>(&body) {
                            full_text.push_str(&schedule::format_lessons(&lessons, weekday, &parity));
                            full_text.push('\n');
                        }
                    }
                }
            }

            if full_text.is_empty() {
                full_text = texts.get("msg.schedule_week_error").to_string();
            }

            let buttons = schedule_buttons(texts);
            let url = sender.build_send_url(chat_id, &full_text, &buttons);
            let req = OutgoingRequest::new(Fields::new());
            let _ = http_client::send(&url, req);
        }
        BotResponse::AskGroup { new_node_id } => {
            if let Some(node_id) = new_node_id {
                let _ = conn.execute(db::UPDATE_USER_NODE, &[Parameter::Int64(node_id), Parameter::Int64(user_id)]);
            }
            let _ = conn.execute(db::CLEAR_USER_GROUP, &[Parameter::Int64(user_id)]);

            let buttons = vec![
                bot_core::messenger::Button { label: texts.get("btn.back").into(), payload: "schedule_back".into() },
                bot_core::messenger::Button { label: texts.get("btn.home").into(), payload: "1".into() },
            ];
            let url = sender.build_send_url(chat_id, texts.get("msg.ask_group"), &buttons);
            let req = OutgoingRequest::new(Fields::new());
            let _ = http_client::send(&url, req);
        }
    }
}

fn schedule_buttons(texts: &Texts) -> Vec<bot_core::messenger::Button> {
    vec![
        bot_core::messenger::Button { label: texts.get("btn.schedule_today").into(), payload: "schedule_today".into() },
        bot_core::messenger::Button { label: texts.get("btn.schedule_tomorrow").into(), payload: "schedule_tomorrow".into() },
        bot_core::messenger::Button { label: texts.get("btn.schedule_this_week").into(), payload: "schedule_this_week".into() },
        bot_core::messenger::Button { label: texts.get("btn.schedule_next_week").into(), payload: "schedule_next_week".into() },
        bot_core::messenger::Button { label: texts.get("btn.schedule_change_group").into(), payload: "schedule_change_group".into() },
        bot_core::messenger::Button { label: texts.get("btn.back").into(), payload: "schedule_back".into() },
        bot_core::messenger::Button { label: texts.get("btn.home").into(), payload: "1".into() },
    ]
}

fn get_or_create_user(conn: &postgres::Connection, platform: &str, platform_user_id: i64) -> (i64, Option<i64>, Option<String>) {
    if let Ok(rows) = conn.query(db::SELECT_USER, &[Parameter::Text(platform.to_string()), Parameter::Int64(platform_user_id)]) {
        if let Some(row) = rows.rows.first() {
            let user_id = match &row[0] { postgres::Value::Int64(id) => *id, _ => return (0, None, None) };
            let node_id = match &row[1] { postgres::Value::Int64(id) => Some(*id), _ => None };
            let group = match &row[2] { postgres::Value::Text(g) => Some(g.clone()), _ => None };
            return (user_id, node_id, group);
        }
    }
    if let Ok(rows) = conn.query(db::UPSERT_USER, &[Parameter::Text(platform.to_string()), Parameter::Int64(platform_user_id)]) {
        if let Some(row) = rows.rows.first() {
            let user_id = match &row[0] { postgres::Value::Int64(id) => *id, _ => 0 };
            return (user_id, None, None);
        }
    }
    (0, None, None)
}

fn load_menu_nodes(conn: &postgres::Connection) -> Vec<MenuNode> {
    let rows = match conn.query(db::SELECT_ALL_MENU_NODES, &[]) { Ok(r) => r, Err(_) => return vec![] };
    rows.rows.iter().filter_map(|row| {
        Some(MenuNode {
            id: match &row[0] { postgres::Value::Int64(v) => *v, _ => return None },
            parent_id: match &row[1] { postgres::Value::Int64(v) => Some(*v), _ => None },
            slug: match &row[2] { postgres::Value::Text(v) => v.clone(), _ => return None },
            title: match &row[3] { postgres::Value::Text(v) => v.clone(), _ => return None },
            content: match &row[4] { postgres::Value::Text(v) => Some(v.clone()), _ => None },
            image_url: match &row[5] { postgres::Value::Text(v) => Some(v.clone()), _ => None },
            sort_order: match &row[6] { postgres::Value::Int32(v) => *v, _ => 0 },
        })
    }).collect()
}

fn load_texts(conn: &postgres::Connection) -> Texts {
    let mut map = HashMap::new();
    if let Ok(rows) = conn.query(db::SELECT_ALL_TEXTS, &[]) {
        for row in &rows.rows {
            if let (Some(postgres::Value::Text(key)), Some(postgres::Value::Text(value))) = (row.get(0), row.get(1)) {
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
            Ok(vec) => { if vec.is_empty() { break; } buf.extend_from_slice(&vec); }
        }
    }
    Ok(buf)
}

fn read_response_body(response: &IncomingResponse) -> Option<Vec<u8>> {
    let body = response.consume().ok()?;
    let stream = body.stream().ok()?;
    let mut buf = Vec::new();
    loop {
        match stream.blocking_read(4096) {
            Err(_) => break,
            Ok(vec) => { if vec.is_empty() { break; } buf.extend_from_slice(&vec); }
        }
    }
    Some(buf)
}

fn write_response(out: ResponseOutparam, status: u16, body_bytes: &[u8]) {
    let res = OutgoingResponse::new(Fields::new());
    res.set_status_code(status).unwrap();
    let body = res.body().unwrap();
    { let stream = body.write().unwrap(); stream.write(body_bytes.into()).unwrap(); }
    OutgoingBody::finish(body, None).unwrap();
    ResponseOutparam::set(out, Ok(res));
}

export!(Plugin);
