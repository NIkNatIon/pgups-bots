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

use bot_core::{
    BotHandler, UserInfo, db, i18n::I18n, menu::MenuNode, messenger::BotResponse, schedule,
};
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

    let (user_id, current_node_id, user_info) = get_or_create_user(&conn, "vk", peer_id);
    let menu_nodes = load_menu_nodes(&conn);
    let i18n = load_i18n(&conn, &user_info.lang);
    let handler = BotHandler::new(menu_nodes, i18n);

    let payload = message.payload.as_deref().map(|p| p.trim_matches('"'));

    let is_in_schedule = current_node_id
        .and_then(|nid| handler.menu_nodes_ref().iter().find(|n| n.id == nid))
        .map(|n| n.slug == "schedule")
        .unwrap_or(false);

    let is_group_input = is_in_schedule
        && payload.is_none()
        && !message.text.is_empty()
        && !message.text.starts_with('/');

    if !user_info.onboarded
        && user_info.role.as_deref() == Some("student")
        && user_info.student_group.is_none()
        && payload.is_none()
        && !message.text.is_empty()
        && !message.text.starts_with('/')
    {
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
            let _ = conn.execute(
                db::UPDATE_USER_ONBOARDED,
                &[Parameter::Boolean(true), Parameter::Int64(user_id)],
            );
            let new_info = UserInfo {
                student_group: Some(group),
                onboarded: true,
                ..user_info
            };
            let response = handler.build_main_menu(&new_info);
            process_response(
                response,
                peer_id,
                &conn,
                user_id,
                vk_token,
                deanery_host,
                &handler,
            );
        } else {
            let sender = VkSender::new(vk_token.to_string());
            let buttons = vec![bot_core::messenger::Button {
                label: handler.i18n().get("btn-skip"),
                payload: "skip_group".into(),
            }];
            let url = sender.build_send_url(
                peer_id,
                &handler.i18n().get("msg-schedule-invalid-group"),
                &buttons,
                None,
            );
            let _ = client::get(url).send();
        }
        return;
    }

    let sender = VkSender::new(vk_token.to_string());

    let settings_group_input = !is_in_schedule
        && user_info.onboarded
        && user_info.student_group.is_none()
        && payload.is_none()
        && !message.text.is_empty()
        && !message.text.starts_with('/');

    if settings_group_input {
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
                &[Parameter::Text(group), Parameter::Int64(user_id)],
            );
            let user_info = get_user_info(&conn, user_id);
            let msg = handler.build_settings(&user_info);
            let url = sender.build_send_url(peer_id, &msg.text, &msg.buttons, None);
            let _ = client::get(url).send();
        } else {
            let buttons = vec![bot_core::messenger::Button {
                label: handler.i18n().get("btn-back"),
                payload: "open_settings".into(),
            }];
            let url = sender.build_send_url(
                peer_id,
                &handler.i18n().get("msg-schedule-invalid-group"),
                &buttons,
                None,
            );
            let _ = client::get(url).send();
        }
        return;
    }

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
            let updated_info = UserInfo {
                student_group: Some(group.clone()),
                ..user_info
            };
            let response = handler.handle_message(current_node_id, None, None, &updated_info);
            process_response(
                response,
                peer_id,
                &conn,
                user_id,
                vk_token,
                deanery_host,
                &handler,
            );
        } else {
            let sender = VkSender::new(vk_token.to_string());
            let buttons = vec![
                bot_core::messenger::Button {
                    label: handler.i18n().get("btn-back"),
                    payload: "schedule_back".into(),
                },
                bot_core::messenger::Button {
                    label: handler.i18n().get("btn-home"),
                    payload: "main_menu".into(),
                },
            ];
            let url = sender.build_send_url(
                peer_id,
                &handler.i18n().get("msg-schedule-invalid-group"),
                &buttons,
                None,
            );
            let _ = client::get(url).send();
        }
        return;
    }

    let response =
        handler.handle_message(current_node_id, payload, Some(&message.text), &user_info);
    process_response(
        response,
        peer_id,
        &conn,
        user_id,
        vk_token,
        deanery_host,
        &handler,
    );
}

fn process_response(
    response: BotResponse,
    peer_id: i64,
    conn: &postgres::Connection,
    user_id: i64,
    vk_token: &str,
    deanery_host: &str,
    handler: &BotHandler,
) {
    let sender = VkSender::new(vk_token.to_string());
    let i18n = handler.i18n();

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
                        Ok(lessons) if lessons.is_empty() => i18n.get("msg-schedule-day-off"),
                        Ok(lessons) => schedule::format_lessons(&lessons, &weekday, &parity, i18n),
                        Err(_) => i18n.get("msg-schedule-error"),
                    }
                }
                Err(_) => i18n.get("msg-schedule-connection-error"),
            };
            let buttons = schedule_buttons(i18n);
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
                        full_text
                            .push_str(&schedule::format_lessons(&lessons, weekday, &parity, i18n));
                        full_text.push('\n');
                    }
                }
            }
            if full_text.is_empty() {
                full_text = i18n.get("msg-schedule-week-error");
            }
            let buttons = schedule_buttons(i18n);
            let url = sender.build_send_url(peer_id, &full_text, &buttons, None);
            let _ = client::get(url).send();
        }
        BotResponse::AskGroup { new_node_id: _ } => {
            if let Some(schedule_node) = handler
                .menu_nodes_ref()
                .iter()
                .find(|n| n.slug == "schedule")
            {
                let _ = conn.execute(
                    db::UPDATE_USER_NODE,
                    &[
                        Parameter::Int64(schedule_node.id),
                        Parameter::Int64(user_id),
                    ],
                );
            }
            let _ = conn.execute(db::CLEAR_USER_GROUP, &[Parameter::Int64(user_id)]);
            let buttons = vec![bot_core::messenger::Button {
                label: i18n.get("btn-back"),
                payload: "main_menu".into(),
            }];
            let url = sender.build_send_url(peer_id, &i18n.get("msg-ask-group"), &buttons, None);
            let _ = client::get(url).send();
        }
        BotResponse::LanguageSelect | BotResponse::OnboardingAskLang => {
            let buttons = vec![
                bot_core::messenger::Button {
                    label: "🇷🇺 Русский".into(),
                    payload: "lang_ru".into(),
                },
                bot_core::messenger::Button {
                    label: "🇬🇧 English".into(),
                    payload: "lang_en".into(),
                },
                bot_core::messenger::Button {
                    label: "🇨🇳 中文".into(),
                    payload: "lang_zh".into(),
                },
            ];
            let url = sender.build_send_url(
                peer_id,
                &i18n.get("msg-onboarding-ask-lang"),
                &buttons,
                None,
            );
            let _ = client::get(url).send();
        }
        BotResponse::LanguageChanged { lang } => {
            let _ = conn.execute(
                db::UPDATE_USER_LANG,
                &[Parameter::Text(lang.clone()), Parameter::Int64(user_id)],
            );
            let new_i18n = load_i18n(conn, &lang);
            let new_handler = BotHandler::new(load_menu_nodes(conn), new_i18n);
            let user_info = get_user_info(conn, user_id);
            if user_info.onboarded {
                let msg = new_handler.build_settings(&user_info);
                let url = sender.build_send_url(peer_id, &msg.text, &msg.buttons, None);
                let _ = client::get(url).send();
            } else {
                let buttons = vec![
                    bot_core::messenger::Button {
                        label: new_handler.i18n().get("role-applicant"),
                        payload: "role_applicant".into(),
                    },
                    bot_core::messenger::Button {
                        label: new_handler.i18n().get("role-student"),
                        payload: "role_student".into(),
                    },
                    bot_core::messenger::Button {
                        label: new_handler.i18n().get("role-teacher"),
                        payload: "role_teacher".into(),
                    },
                    bot_core::messenger::Button {
                        label: new_handler.i18n().get("role-guest"),
                        payload: "role_guest".into(),
                    },
                ];
                let url = sender.build_send_url(
                    peer_id,
                    &new_handler.i18n().get("msg-onboarding-ask-role"),
                    &buttons,
                    None,
                );
                let _ = client::get(url).send();
            }
        }
        BotResponse::OnboardingAskRole => {
            let buttons = vec![
                bot_core::messenger::Button {
                    label: i18n.get("role-applicant"),
                    payload: "role_applicant".into(),
                },
                bot_core::messenger::Button {
                    label: i18n.get("role-student"),
                    payload: "role_student".into(),
                },
                bot_core::messenger::Button {
                    label: i18n.get("role-teacher"),
                    payload: "role_teacher".into(),
                },
                bot_core::messenger::Button {
                    label: i18n.get("role-guest"),
                    payload: "role_guest".into(),
                },
            ];
            let url = sender.build_send_url(
                peer_id,
                &i18n.get("msg-onboarding-ask-role"),
                &buttons,
                None,
            );
            let _ = client::get(url).send();
        }
        BotResponse::RoleChanged { role } => {
            let _ = conn.execute(
                db::UPDATE_USER_ROLE,
                &[Parameter::Text(role.clone()), Parameter::Int64(user_id)],
            );
            let user_info = get_user_info(conn, user_id);
            if user_info.onboarded {
                let msg = handler.build_settings(&user_info);
                let url = sender.build_send_url(peer_id, &msg.text, &msg.buttons, None);
                let _ = client::get(url).send();
            } else if role == "student" {
                let buttons = vec![bot_core::messenger::Button {
                    label: i18n.get("btn-skip"),
                    payload: "skip_group".into(),
                }];
                let url = sender.build_send_url(
                    peer_id,
                    &i18n.get("msg-onboarding-ask-group"),
                    &buttons,
                    None,
                );
                let _ = client::get(url).send();
            } else {
                let _ = conn.execute(
                    db::UPDATE_USER_ONBOARDED,
                    &[Parameter::Boolean(true), Parameter::Int64(user_id)],
                );
                let new_info = UserInfo {
                    role: Some(role),
                    onboarded: true,
                    lang: String::new(),
                    student_group: None,
                };
                let response = handler.build_main_menu(&new_info);
                process_response(
                    response,
                    peer_id,
                    conn,
                    user_id,
                    vk_token,
                    deanery_host,
                    handler,
                );
            }
        }
        BotResponse::OnboardingAskGroup => {
            let _ = conn.execute(
                db::UPDATE_USER_ONBOARDED,
                &[Parameter::Boolean(true), Parameter::Int64(user_id)],
            );
            let new_info = UserInfo {
                role: Some("student".into()),
                onboarded: true,
                lang: String::new(),
                student_group: None,
            };
            let response = handler.build_main_menu(&new_info);
            process_response(
                response,
                peer_id,
                conn,
                user_id,
                vk_token,
                deanery_host,
                handler,
            );
        }
        BotResponse::SettingsChangeGroup => {
            let _ = conn.execute(db::CLEAR_USER_GROUP, &[Parameter::Int64(user_id)]);
            let buttons = vec![bot_core::messenger::Button {
                label: i18n.get("btn-back"),
                payload: "open_settings".into(),
            }];
            let url = sender.build_send_url(peer_id, &i18n.get("msg-ask-group"), &buttons, None);
            let _ = client::get(url).send();
        }
        BotResponse::SettingsGroupChanged { group } => {
            let _ = conn.execute(
                db::UPDATE_USER_GROUP,
                &[Parameter::Text(group), Parameter::Int64(user_id)],
            );
            let user_info = get_user_info(conn, user_id);
            let msg = handler.build_settings(&user_info);
            let url = sender.build_send_url(peer_id, &msg.text, &msg.buttons, None);
            let _ = client::get(url).send();
        }
        BotResponse::Settings => {
            let user_info = get_user_info(conn, user_id);
            let msg = handler.build_settings(&user_info);
            let url = sender.build_send_url(peer_id, &msg.text, &msg.buttons, None);
            let _ = client::get(url).send();
        }
    }
}

fn schedule_buttons(i18n: &I18n) -> Vec<bot_core::messenger::Button> {
    vec![
        bot_core::messenger::Button {
            label: i18n.get("btn-schedule-today"),
            payload: "schedule_today".into(),
        },
        bot_core::messenger::Button {
            label: i18n.get("btn-schedule-tomorrow"),
            payload: "schedule_tomorrow".into(),
        },
        bot_core::messenger::Button {
            label: i18n.get("btn-schedule-this-week"),
            payload: "schedule_this_week".into(),
        },
        bot_core::messenger::Button {
            label: i18n.get("btn-schedule-next-week"),
            payload: "schedule_next_week".into(),
        },
        bot_core::messenger::Button {
            label: i18n.get("btn-schedule-change-group"),
            payload: "schedule_change_group".into(),
        },
        bot_core::messenger::Button {
            label: i18n.get("btn-back"),
            payload: "main_menu".into(),
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

    let headers = wassel_sdk::bindings::wasi::http::types::Fields::new();
    let _ = headers.set("content-type", &[content_type.as_bytes().to_vec()]);
    let req = wassel_sdk::bindings::wasi::http::types::OutgoingRequest::new(headers);
    let _ = req.set_method(&wassel_sdk::bindings::wasi::http::types::Method::Post);
    let req_body = req.body().unwrap();
    {
        let stream = req_body.write().unwrap();
        let _ = stream.write(&multipart_body);
    }
    wassel_sdk::bindings::wasi::http::types::OutgoingBody::finish(req_body, None).unwrap();
    let resp =
        wassel_sdk::bindings::wassel::foundation::http_client::send(&upload_url, req).ok()?;

    let body = resp.consume().ok()?;
    let in_stream = body.stream().ok()?;
    let mut buf = Vec::new();
    loop {
        match in_stream.blocking_read(4096) {
            Err(_) => break,
            Ok(vec) => {
                if vec.is_empty() {
                    break;
                }
                buf.extend_from_slice(&vec);
            }
        }
    }
    let upload_result: photo::UploadResult = serde_json::from_slice(&buf).ok()?;

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
) -> (i64, Option<i64>, UserInfo) {
    let default_info = UserInfo {
        lang: String::new(),
        role: None,
        student_group: None,
        onboarded: false,
    };

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
            _ => return (0, None, default_info),
        };
        let node_id = match &row[1] {
            postgres::Value::Int64(id) => Some(*id),
            _ => None,
        };
        let group = match &row[2] {
            postgres::Value::Text(g) => Some(g.clone()),
            _ => None,
        };
        let lang = match &row[3] {
            postgres::Value::Text(l) => l.clone(),
            _ => String::new(),
        };
        let role = match &row[4] {
            postgres::Value::Text(r) => Some(r.clone()),
            _ => None,
        };
        let onboarded = match &row[5] {
            postgres::Value::Boolean(b) => *b,
            _ => false,
        };
        return (
            user_id,
            node_id,
            UserInfo {
                lang,
                role,
                student_group: group,
                onboarded,
            },
        );
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
        return (user_id, None, default_info);
    }
    (0, None, default_info)
}

fn get_user_info(conn: &postgres::Connection, user_id: i64) -> UserInfo {
    if let Ok(rows) = conn.query(
        "SELECT student_group, lang, role, onboarded FROM users WHERE id = $1",
        &[Parameter::Int64(user_id)],
    ) && let Some(row) = rows.rows.first()
    {
        let group = match &row[0] {
            postgres::Value::Text(g) => Some(g.clone()),
            _ => None,
        };
        let lang = match &row[1] {
            postgres::Value::Text(l) => l.clone(),
            _ => "ru".into(),
        };
        let role = match &row[2] {
            postgres::Value::Text(r) => Some(r.clone()),
            _ => None,
        };
        let onboarded = match &row[3] {
            postgres::Value::Boolean(b) => *b,
            _ => false,
        };
        return UserInfo {
            lang,
            role,
            student_group: group,
            onboarded,
        };
    }
    UserInfo {
        lang: "ru".into(),
        role: None,
        student_group: None,
        onboarded: false,
    }
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
                image_url: match &row[3] {
                    postgres::Value::Text(v) => Some(v.clone()),
                    _ => None,
                },
                sort_order: match &row[4] {
                    postgres::Value::Int32(v) => *v,
                    _ => 0,
                },
            })
        })
        .collect()
}

fn load_i18n(conn: &postgres::Connection, lang: &str) -> I18n {
    let actual_lang = if lang.is_empty() { "ru" } else { lang };
    if let Ok(rows) = conn.query(
        db::SELECT_TRANSLATION,
        &[Parameter::Text(actual_lang.to_string())],
    ) && let Some(row) = rows.rows.first()
        && let postgres::Value::Text(content) = &row[0]
    {
        return I18n::new(actual_lang, content);
    }
    I18n::new("ru", "")
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
