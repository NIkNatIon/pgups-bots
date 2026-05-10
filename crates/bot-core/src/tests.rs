use super::*;
use crate::menu::MenuNode;

fn test_menu() -> Vec<MenuNode> {
    vec![
        MenuNode { id: 1, parent_id: None, slug: "start".into(), title: "Главное меню".into(), content: Some("Добро пожаловать!".into()), image_url: None, sort_order: 0 },
        MenuNode { id: 2, parent_id: Some(1), slug: "info".into(), title: "Информация".into(), content: Some("Выберите раздел:".into()), image_url: None, sort_order: 0 },
        MenuNode { id: 3, parent_id: Some(1), slug: "about".into(), title: "О проекте".into(), content: Some("Описание проекта".into()), image_url: None, sort_order: 1 },
        MenuNode { id: 10, parent_id: Some(2), slug: "abit".into(), title: "Абитуриентам".into(), content: Some("Инфо для абитуриентов".into()), image_url: None, sort_order: 0 },
        MenuNode { id: 11, parent_id: Some(2), slug: "stud".into(), title: "Студентам".into(), content: Some("Инфо для студентов".into()), image_url: None, sort_order: 1 },
        MenuNode { id: 20, parent_id: Some(10), slug: "docs".into(), title: "Документы".into(), content: Some("Список документов".into()), image_url: None, sort_order: 0 },
        MenuNode { id: 21, parent_id: Some(10), slug: "map".into(), title: "Карта".into(), content: None, image_url: Some("karta.jpg".into()), sort_order: 1 },
    ]
}

#[test]
fn navigate_to_root() {
    let handler = BotHandler::new(test_menu());
    let (msg, node_id) = handler.handle_message(None, None, None);
    assert_eq!(node_id, Some(1));
    assert_eq!(msg.text, "Добро пожаловать!");
    assert_eq!(msg.buttons.len(), 2);
    assert_eq!(msg.buttons[0].label, "Информация");
    assert_eq!(msg.buttons[1].label, "О проекте");
}

#[test]
fn navigate_by_payload() {
    let handler = BotHandler::new(test_menu());
    let (msg, node_id) = handler.handle_message(Some(1), Some("2"), None);
    assert_eq!(node_id, Some(2));
    assert_eq!(msg.text, "Выберите раздел:");
    assert_eq!(msg.buttons.len(), 3);
    assert_eq!(msg.buttons[0].label, "Абитуриентам");
    assert_eq!(msg.buttons[1].label, "Студентам");
    assert_eq!(msg.buttons[2].label, "⬅ Назад");
}

#[test]
fn navigate_deep_has_home_button() {
    let handler = BotHandler::new(test_menu());
    let (msg, node_id) = handler.handle_message(Some(2), Some("10"), None);
    assert_eq!(node_id, Some(10));
    assert!(msg.buttons.iter().any(|b| b.label == "⬅ Назад"));
    assert!(msg.buttons.iter().any(|b| b.label == "🏠 В начало"));
}

#[test]
fn navigate_first_level_no_home_button() {
    let handler = BotHandler::new(test_menu());
    let (msg, _) = handler.handle_message(Some(1), Some("2"), None);
    assert!(msg.buttons.iter().any(|b| b.label == "⬅ Назад"));
    assert!(!msg.buttons.iter().any(|b| b.label == "🏠 В начало"));
}

#[test]
fn navigate_by_text() {
    let handler = BotHandler::new(test_menu());
    let (msg, node_id) = handler.handle_message(Some(1), None, Some("О проекте"));
    assert_eq!(node_id, Some(3));
    assert_eq!(msg.text, "Описание проекта");
}

#[test]
fn back_button_payload() {
    let handler = BotHandler::new(test_menu());
    let (msg, _) = handler.handle_message(Some(1), Some("2"), None);
    let back = msg.buttons.iter().find(|b| b.label == "⬅ Назад").unwrap();
    assert_eq!(back.payload, "1");
}

#[test]
fn unknown_payload_goes_to_root() {
    let handler = BotHandler::new(test_menu());
    let (msg, node_id) = handler.handle_message(Some(1), Some("999"), None);
    assert_eq!(node_id, Some(1));
    assert_eq!(msg.text, "Добро пожаловать!");
}

#[test]
fn image_url_passed_through() {
    let handler = BotHandler::new(test_menu());
    let (msg, _) = handler.handle_message(Some(10), Some("21"), None);
    assert_eq!(msg.image_url, Some("karta.jpg".into()));
}

#[test]
fn content_fallback_to_title() {
    let handler = BotHandler::new(test_menu());
    let (msg, _) = handler.handle_message(Some(10), Some("21"), None);
    assert_eq!(msg.text, "Карта");
}

#[test]
fn leaf_node_has_only_nav_buttons() {
    let handler = BotHandler::new(test_menu());
    let (msg, _) = handler.handle_message(Some(10), Some("20"), None);
    assert_eq!(msg.text, "Список документов");
    assert!(msg.buttons.iter().all(|b| b.label == "⬅ Назад" || b.label == "🏠 В начало"));
}