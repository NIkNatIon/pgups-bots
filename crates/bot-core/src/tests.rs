use super::*;
use crate::menu::MenuNode;
use crate::messenger::BotResponse;
use crate::texts::Texts;
use std::collections::HashMap;

fn test_texts() -> Texts {
    let mut map = HashMap::new();
    map.insert("btn.back".into(), "⬅ Назад".into());
    map.insert("btn.home".into(), "🏠 В начало".into());
    map.insert("btn.schedule_today".into(), "На сегодня".into());
    map.insert("btn.schedule_tomorrow".into(), "На завтра".into());
    map.insert("btn.schedule_this_week".into(), "Текущая неделя".into());
    map.insert("btn.schedule_next_week".into(), "Следующая неделя".into());
    map.insert(
        "btn.schedule_change_group".into(),
        "🔄 Сменить группу".into(),
    );
    map.insert("msg.select_section".into(), "Выберите раздел:".into());
    map.insert(
        "msg.ask_group".into(),
        "Введите номер вашей группы (например, ИВБ-211):".into(),
    );
    map.insert(
        "msg.schedule_header".into(),
        "Расписание для группы {group}\n\nСегодня: {weekday}, {parity} неделя".into(),
    );
    Texts::new(map)
}

fn test_menu() -> Vec<MenuNode> {
    vec![
        MenuNode {
            id: 1,
            parent_id: None,
            slug: "start".into(),
            title: "Главное меню".into(),
            content: Some("Добро пожаловать!".into()),
            image_url: None,
            sort_order: 0,
        },
        MenuNode {
            id: 2,
            parent_id: Some(1),
            slug: "info".into(),
            title: "Информация".into(),
            content: Some("Выберите раздел:".into()),
            image_url: None,
            sort_order: 0,
        },
        MenuNode {
            id: 3,
            parent_id: Some(1),
            slug: "about".into(),
            title: "О проекте".into(),
            content: Some("Описание проекта".into()),
            image_url: None,
            sort_order: 1,
        },
        MenuNode {
            id: 10,
            parent_id: Some(2),
            slug: "abit".into(),
            title: "Абитуриентам".into(),
            content: Some("Инфо для абитуриентов".into()),
            image_url: None,
            sort_order: 0,
        },
        MenuNode {
            id: 11,
            parent_id: Some(2),
            slug: "stud".into(),
            title: "Студентам".into(),
            content: Some("Инфо для студентов".into()),
            image_url: None,
            sort_order: 1,
        },
        MenuNode {
            id: 20,
            parent_id: Some(10),
            slug: "docs".into(),
            title: "Документы".into(),
            content: Some("Список документов".into()),
            image_url: None,
            sort_order: 0,
        },
        MenuNode {
            id: 21,
            parent_id: Some(10),
            slug: "map".into(),
            title: "Карта".into(),
            content: None,
            image_url: Some("karta.jpg".into()),
            sort_order: 1,
        },
        MenuNode {
            id: 30,
            parent_id: Some(11),
            slug: "schedule".into(),
            title: "Расписание".into(),
            content: None,
            image_url: None,
            sort_order: 0,
        },
    ]
}

fn unwrap_message(response: BotResponse) -> (OutgoingMessage, Option<i64>) {
    match response {
        BotResponse::Message(msg, node_id) => (msg, node_id),
        other => panic!("Expected BotResponse::Message, got {:?}", other),
    }
}

#[test]
fn navigate_to_root() {
    let handler = BotHandler::new(test_menu(), test_texts());
    let (msg, node_id) = unwrap_message(handler.handle_message(None, None, None, None));
    assert_eq!(node_id, Some(1));
    assert_eq!(msg.text, "Добро пожаловать!");
    assert_eq!(msg.buttons.len(), 2);
}

#[test]
fn navigate_by_payload() {
    let handler = BotHandler::new(test_menu(), test_texts());
    let (msg, node_id) = unwrap_message(handler.handle_message(Some(1), Some("2"), None, None));
    assert_eq!(node_id, Some(2));
    assert_eq!(msg.buttons.len(), 3);
    assert_eq!(msg.buttons[2].label, "⬅ Назад");
}

#[test]
fn navigate_deep_has_home_button() {
    let handler = BotHandler::new(test_menu(), test_texts());
    let (msg, _) = unwrap_message(handler.handle_message(Some(2), Some("10"), None, None));
    assert!(msg.buttons.iter().any(|b| b.label == "⬅ Назад"));
    assert!(msg.buttons.iter().any(|b| b.label == "🏠 В начало"));
}

#[test]
fn navigate_first_level_no_home_button() {
    let handler = BotHandler::new(test_menu(), test_texts());
    let (msg, _) = unwrap_message(handler.handle_message(Some(1), Some("2"), None, None));
    assert!(msg.buttons.iter().any(|b| b.label == "⬅ Назад"));
    assert!(!msg.buttons.iter().any(|b| b.label == "🏠 В начало"));
}

#[test]
fn navigate_by_text() {
    let handler = BotHandler::new(test_menu(), test_texts());
    let (msg, node_id) =
        unwrap_message(handler.handle_message(Some(1), None, Some("О проекте"), None));
    assert_eq!(node_id, Some(3));
    assert_eq!(msg.text, "Описание проекта");
}

#[test]
fn back_button_payload() {
    let handler = BotHandler::new(test_menu(), test_texts());
    let (msg, _) = unwrap_message(handler.handle_message(Some(1), Some("2"), None, None));
    let back = msg.buttons.iter().find(|b| b.label == "⬅ Назад").unwrap();
    assert_eq!(back.payload, "1");
}

#[test]
fn unknown_payload_goes_to_root() {
    let handler = BotHandler::new(test_menu(), test_texts());
    let (msg, node_id) = unwrap_message(handler.handle_message(Some(1), Some("999"), None, None));
    assert_eq!(node_id, Some(1));
    assert_eq!(msg.text, "Добро пожаловать!");
}

#[test]
fn image_url_passed_through() {
    let handler = BotHandler::new(test_menu(), test_texts());
    let (msg, _) = unwrap_message(handler.handle_message(Some(10), Some("21"), None, None));
    assert_eq!(msg.image_url, Some("karta.jpg".into()));
}

#[test]
fn content_fallback_to_title() {
    let handler = BotHandler::new(test_menu(), test_texts());
    let (msg, _) = unwrap_message(handler.handle_message(Some(10), Some("21"), None, None));
    assert_eq!(msg.text, "Карта");
}

#[test]
fn leaf_node_has_only_nav_buttons() {
    let handler = BotHandler::new(test_menu(), test_texts());
    let (msg, _) = unwrap_message(handler.handle_message(Some(10), Some("20"), None, None));
    assert!(
        msg.buttons
            .iter()
            .all(|b| b.label == "⬅ Назад" || b.label == "🏠 В начало")
    );
}

#[test]
fn schedule_without_group_asks_for_group() {
    let handler = BotHandler::new(test_menu(), test_texts());
    let (msg, node_id) = unwrap_message(handler.handle_message(Some(11), Some("30"), None, None));
    assert_eq!(node_id, Some(30));
    assert!(msg.text.contains("Введите номер"));
}

#[test]
fn schedule_with_group_shows_menu() {
    let handler = BotHandler::new(test_menu(), test_texts());
    let (msg, node_id) =
        unwrap_message(handler.handle_message(Some(11), Some("30"), None, Some("БИБ-512")));
    assert_eq!(node_id, Some(30));
    assert!(msg.text.contains("БИБ-512"));
    assert!(msg.buttons.iter().any(|b| b.payload == "schedule_today"));
}

#[test]
fn schedule_today_returns_request() {
    let handler = BotHandler::new(test_menu(), test_texts());
    let response = handler.handle_message(Some(30), Some("schedule_today"), None, Some("БИБ-512"));
    match response {
        BotResponse::ScheduleRequest { group, .. } => assert_eq!(group, "БИБ-512"),
        other => panic!("Expected ScheduleRequest, got {:?}", other),
    }
}

#[test]
fn schedule_today_without_group_asks() {
    let handler = BotHandler::new(test_menu(), test_texts());
    let response = handler.handle_message(Some(30), Some("schedule_today"), None, None);
    match response {
        BotResponse::AskGroup { .. } => {}
        other => panic!("Expected AskGroup, got {:?}", other),
    }
}

#[test]
fn texts_format_works() {
    let texts = test_texts();
    let result = texts.format(
        "msg.schedule_header",
        &[
            ("group", "БИБ-512"),
            ("weekday", "Понедельник"),
            ("parity", "чётная"),
        ],
    );
    assert!(result.contains("БИБ-512"));
    assert!(result.contains("Понедельник"));
    assert!(result.contains("чётная"));
}
