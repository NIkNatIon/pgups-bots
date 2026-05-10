use std::fs;

use wassel_sdk_rust::bindings::{
    export,
    exports::wassel::foundation::http_handler::Guest,
    wasi::http::types::{
        Fields, IncomingRequest, Method, OutgoingBody, OutgoingRequest,
        OutgoingResponse, ResponseOutparam,
    },
    wasi::io::streams::StreamError,
    wassel::foundation::{http_client, postgres::{self, Parameter}},
};

use bot_core::{BotHandler, db, menu::MenuNode};
use tg_api::{sender::TgSender, webhook::Update};

struct Plugin;

fn get_config(key: &str) -> String {
    wassel_sdk_rust::bindings::wasi::config::store::get(key)
        .unwrap_or_default()
        .unwrap_or_default()
}

impl Guest for Plugin {
    fn handle_request(request: IncomingRequest, response_out: ResponseOutparam) {
        let db_connection = get_config("db_connection");
        let tg_bot_token = get_config("tg_bot_token");

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

        handle_update(update, &db_connection, &tg_bot_token);

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

fn handle_update(update: Update, db_connection: &str, tg_bot_token: &str) {
    let (chat_id, payload, text, callback_query_id) = match update.extract() {
        Some(data) => data,
        None => return,
    };

    let config = postgres::ConnectionConfig::new(db_connection);
    let conn = match postgres::Connection::open(config) {
        Ok(c) => c,
        Err(_) => return,
    };

    let (user_id, current_node_id) = get_or_create_user(&conn, "tg", chat_id);

    let menu_nodes = load_menu_nodes(&conn);

    let handler = BotHandler::new(menu_nodes);

    let (response, new_node_id) = handler.handle_message(
        current_node_id,
        payload,
        text,
    );

    if let Some(node_id) = new_node_id {
        let _ = conn.execute(
            db::UPDATE_USER_NODE,
            &[Parameter::Int64(node_id), Parameter::Int64(user_id)],
        );
    }

    if let Some(cb_id) = callback_query_id {
        let sender = TgSender::new(tg_bot_token.to_string());
        let url = sender.build_answer_callback_url(cb_id);
        let req = OutgoingRequest::new(Fields::new());
        let _ = http_client::send(&url, req);
    }

    let sender = TgSender::new(tg_bot_token.to_string());

    if let Some(image_path) = response.image_url.as_deref() {
        if let Ok(photo_bytes) = fs::read(image_path) {
            let (content_type, multipart_body) = sender.build_send_photo_body(
                chat_id,
                &photo_bytes,
                &response.text,
                &response.buttons,
            );

            let headers = Fields::new();
            headers.set(&"content-type".to_string(), &[content_type.as_bytes().to_vec()]).unwrap();
            let req = OutgoingRequest::new(headers);
            req.set_method(&Method::Post).unwrap();
            let req_body = req.body().unwrap();
            {
                let stream = req_body.write().unwrap();
                stream.write(&multipart_body).unwrap();
            }
            OutgoingBody::finish(req_body, None).unwrap();
            let _ = http_client::send(&sender.send_photo_url(), req);
        }
    } else {
        let url = sender.build_send_url(chat_id, &response.text, &response.buttons);
        let req = OutgoingRequest::new(Fields::new());
        let _ = http_client::send(&url, req);
    }
}

fn get_or_create_user(conn: &postgres::Connection, platform: &str, platform_user_id: i64) -> (i64, Option<i64>) {
    if let Ok(rows) = conn.query(
        db::SELECT_USER,
        &[Parameter::Text(platform.to_string()), Parameter::Int64(platform_user_id)],
    ) {
        if let Some(row) = rows.rows.first() {
            let user_id = match &row[0] {
                postgres::Value::Int64(id) => *id,
                _ => return (0, None),
            };
            let node_id = match &row[1] {
                postgres::Value::Int64(id) => Some(*id),
                postgres::Value::PgNull => None,
                _ => None,
            };
            return (user_id, node_id);
        }
    }

    if let Ok(rows) = conn.query(
        db::UPSERT_USER,
        &[Parameter::Text(platform.to_string()), Parameter::Int64(platform_user_id)],
    ) {
        if let Some(row) = rows.rows.first() {
            let user_id = match &row[0] {
                postgres::Value::Int64(id) => *id,
                _ => 0,
            };
            return (user_id, None);
        }
    }

    (0, None)
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
                id: match &row[0] { postgres::Value::Int64(v) => *v, _ => return None },
                parent_id: match &row[1] { postgres::Value::Int64(v) => Some(*v), _ => None },
                slug: match &row[2] { postgres::Value::Text(v) => v.clone(), _ => return None },
                title: match &row[3] { postgres::Value::Text(v) => v.clone(), _ => return None },
                content: match &row[4] { postgres::Value::Text(v) => Some(v.clone()), _ => None },
                image_url: match &row[5] { postgres::Value::Text(v) => Some(v.clone()), _ => None },
                sort_order: match &row[6] { postgres::Value::Int32(v) => *v, _ => 0 },
            })
        })
        .collect()
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

fn write_response(out: ResponseOutparam, status: u16, body_bytes: &[u8]) {
    let res = OutgoingResponse::new(Fields::new());
    res.set_status_code(status).unwrap();

    let body = res.body().unwrap();
    {
        let stream = body.write().unwrap();
        stream.write(body_bytes.into()).unwrap();
    }
    OutgoingBody::finish(body, None).unwrap();

    ResponseOutparam::set(out, Ok(res));
}

export!(Plugin);