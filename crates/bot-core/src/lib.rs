pub mod db;
pub mod i18n;
pub mod menu;
pub mod messenger;
pub mod schedule;

// #[cfg(test)]
// mod tests;

use i18n::I18n;
use menu::MenuNode;
use messenger::{BotResponse, Button, OutgoingMessage};
use schedule::{current_parity, current_weekday, next_week_parity, tomorrow_weekday};

pub struct BotHandler {
    menu_nodes: Vec<MenuNode>,
    i18n: I18n,
}

impl BotHandler {
    pub fn new(menu_nodes: Vec<MenuNode>, i18n: I18n) -> Self {
        Self { menu_nodes, i18n }
    }

    pub fn menu_nodes_ref(&self) -> &[MenuNode] {
        &self.menu_nodes
    }

    pub fn i18n(&self) -> &I18n {
        &self.i18n
    }

    pub fn handle_message(
        &self,
        user_node_id: Option<i64>,
        payload: Option<&str>,
        text: Option<&str>,
        student_group: Option<&str>,
    ) -> BotResponse {
        if let Some(t) = text
            && t == "/start"
        {
            let (msg, nid) = self.navigate_to_root();
            return BotResponse::Message(msg, nid);
        }

        if let Some(p) = payload {
            match p {
                "change_language" => return BotResponse::LanguageSelect,
                "lang_ru" | "lang_en" | "lang_zh" => {
                    let lang = p.strip_prefix("lang_").unwrap_or("ru");
                    return BotResponse::LanguageChanged {
                        lang: lang.to_string(),
                    };
                }
                "schedule_today" => {
                    return if let Some(group) = student_group {
                        BotResponse::ScheduleRequest {
                            group: group.to_string(),
                            weekday: current_weekday().to_string(),
                            parity: current_parity().to_string(),
                            new_node_id: user_node_id,
                        }
                    } else {
                        BotResponse::AskGroup {
                            new_node_id: user_node_id,
                        }
                    };
                }
                "schedule_tomorrow" => {
                    return if let Some(group) = student_group {
                        let tomorrow = tomorrow_weekday();
                        let parity = if tomorrow == "Monday" {
                            next_week_parity()
                        } else {
                            current_parity()
                        };
                        BotResponse::ScheduleRequest {
                            group: group.to_string(),
                            weekday: tomorrow.to_string(),
                            parity: parity.to_string(),
                            new_node_id: user_node_id,
                        }
                    } else {
                        BotResponse::AskGroup {
                            new_node_id: user_node_id,
                        }
                    };
                }
                "schedule_this_week" => {
                    return if let Some(group) = student_group {
                        BotResponse::ScheduleWeekRequest {
                            group: group.to_string(),
                            parity: current_parity().to_string(),
                            new_node_id: user_node_id,
                        }
                    } else {
                        BotResponse::AskGroup {
                            new_node_id: user_node_id,
                        }
                    };
                }
                "schedule_next_week" => {
                    return if let Some(group) = student_group {
                        BotResponse::ScheduleWeekRequest {
                            group: group.to_string(),
                            parity: next_week_parity().to_string(),
                            new_node_id: user_node_id,
                        }
                    } else {
                        BotResponse::AskGroup {
                            new_node_id: user_node_id,
                        }
                    };
                }
                "schedule_change_group" => {
                    return BotResponse::AskGroup {
                        new_node_id: user_node_id,
                    };
                }
                _ => {}
            }
        }

        if let Some(node_id) = user_node_id
            && let Some(node) = self.menu_nodes.iter().find(|n| n.id == node_id)
            && node.slug == "schedule"
            && payload.is_none()
        {
            if let Some(group_text) = text
                && !group_text.is_empty()
            {
                return BotResponse::Message(self.schedule_menu(Some(group_text)), Some(node_id));
            }
            if student_group.is_some() {
                return BotResponse::Message(self.schedule_menu(student_group), Some(node_id));
            }
        }

        let target_node_id = payload
            .and_then(|p| p.parse::<i64>().ok())
            .or_else(|| self.find_node_by_text(user_node_id, text));

        match target_node_id {
            Some(node_id) => {
                if let Some(node) = self.menu_nodes.iter().find(|n| n.id == node_id)
                    && node.slug == "schedule"
                {
                    return BotResponse::Message(self.schedule_menu(student_group), Some(node_id));
                }
                let (msg, nid) = self.navigate_to(node_id);
                BotResponse::Message(msg, nid)
            }
            None => {
                let (msg, nid) = self.navigate_to_root();
                BotResponse::Message(msg, nid)
            }
        }
    }

    fn schedule_menu(&self, student_group: Option<&str>) -> OutgoingMessage {
        match student_group {
            Some(group) => {
                let weekday_key = format!("schedule-weekday-{}", current_weekday().to_lowercase());
                let parity_key = format!("schedule-parity-{}", current_parity().to_lowercase());
                let weekday = self.i18n.get(&weekday_key);
                let parity = self.i18n.get(&parity_key);

                OutgoingMessage {
                    text: self.i18n.format(
                        "msg-schedule-header",
                        &[("group", group), ("weekday", &weekday), ("parity", &parity)],
                    ),
                    buttons: vec![
                        Button {
                            label: self.i18n.get("btn-schedule-today"),
                            payload: "schedule_today".into(),
                        },
                        Button {
                            label: self.i18n.get("btn-schedule-tomorrow"),
                            payload: "schedule_tomorrow".into(),
                        },
                        Button {
                            label: self.i18n.get("btn-schedule-this-week"),
                            payload: "schedule_this_week".into(),
                        },
                        Button {
                            label: self.i18n.get("btn-schedule-next-week"),
                            payload: "schedule_next_week".into(),
                        },
                        Button {
                            label: self.i18n.get("btn-schedule-change-group"),
                            payload: "schedule_change_group".into(),
                        },
                        Button {
                            label: self.i18n.get("btn-back"),
                            payload: self.find_schedule_parent_id(),
                        },
                        Button {
                            label: self.i18n.get("btn-home"),
                            payload: self
                                .get_roots()
                                .first()
                                .map(|r| r.id.to_string())
                                .unwrap_or("1".into()),
                        },
                    ],
                    image_url: None,
                }
            }
            None => OutgoingMessage {
                text: self.i18n.get("msg-ask-group"),
                buttons: vec![
                    Button {
                        label: self.i18n.get("btn-back"),
                        payload: self.find_schedule_parent_id(),
                    },
                    Button {
                        label: self.i18n.get("btn-home"),
                        payload: self
                            .get_roots()
                            .first()
                            .map(|r| r.id.to_string())
                            .unwrap_or("1".into()),
                    },
                ],
                image_url: None,
            },
        }
    }

    fn find_schedule_parent_id(&self) -> String {
        self.menu_nodes
            .iter()
            .find(|n| n.slug == "schedule")
            .and_then(|n| n.parent_id)
            .map(|id| id.to_string())
            .unwrap_or_else(|| "1".into())
    }

    fn navigate_to(&self, node_id: i64) -> (OutgoingMessage, Option<i64>) {
        let node = match self.menu_nodes.iter().find(|n| n.id == node_id) {
            Some(n) => n,
            None => return self.navigate_to_root(),
        };

        let title_key = format!("menu-{}-title", node.slug);
        let content_key = format!("menu-{}-content", node.slug);

        let title = self.i18n.get(&title_key);
        let content = self.i18n.get(&content_key);

        let display_text = if content != content_key {
            content
        } else {
            title.clone()
        };

        let children = self.get_children(node.id);
        let mut buttons: Vec<Button> = children
            .iter()
            .map(|child| {
                let child_title_key = format!("menu-{}-title", child.slug);
                Button {
                    label: self.i18n.get(&child_title_key),
                    payload: child.id.to_string(),
                }
            })
            .collect();

        if let Some(parent_id) = node.parent_id {
            buttons.push(Button {
                label: self.i18n.get("btn-back"),
                payload: parent_id.to_string(),
            });

            let parent = self.menu_nodes.iter().find(|n| n.id == parent_id);
            if parent.is_some_and(|p| p.parent_id.is_some())
                && let Some(root) = self.get_roots().first()
            {
                buttons.push(Button {
                    label: self.i18n.get("btn-home"),
                    payload: root.id.to_string(),
                });
            }
        }

        if node.parent_id.is_none() {
            buttons.push(Button {
                label: self.i18n.get("btn-language"),
                payload: "change_language".to_string(),
            });
        }

        let msg = OutgoingMessage {
            text: display_text,
            buttons,
            image_url: node.image_url.clone(),
        };

        (msg, Some(node.id))
    }

    fn navigate_to_root(&self) -> (OutgoingMessage, Option<i64>) {
        let roots = self.get_roots();

        if roots.len() == 1 {
            return self.navigate_to(roots[0].id);
        }

        let buttons = roots
            .iter()
            .map(|node| {
                let title_key = format!("menu-{}-title", node.slug);
                Button {
                    label: self.i18n.get(&title_key),
                    payload: node.id.to_string(),
                }
            })
            .collect();

        let msg = OutgoingMessage {
            text: self.i18n.get("msg-select-section"),
            buttons,
            image_url: None,
        };

        (msg, None)
    }

    fn find_node_by_text(&self, current_node_id: Option<i64>, text: Option<&str>) -> Option<i64> {
        let text = text?.trim().to_lowercase();
        let children = match current_node_id {
            Some(id) => self.get_children(id),
            None => self.get_roots(),
        };

        children
            .iter()
            .find(|n| {
                let title_key = format!("menu-{}-title", n.slug);
                self.i18n.get(&title_key).to_lowercase() == text
            })
            .map(|n| n.id)
    }

    fn get_children(&self, parent_id: i64) -> Vec<&MenuNode> {
        let mut children: Vec<&MenuNode> = self
            .menu_nodes
            .iter()
            .filter(|n| n.parent_id == Some(parent_id))
            .collect();
        children.sort_by_key(|n| n.sort_order);
        children
    }

    fn get_roots(&self) -> Vec<&MenuNode> {
        let mut roots: Vec<&MenuNode> = self
            .menu_nodes
            .iter()
            .filter(|n| n.parent_id.is_none())
            .collect();
        roots.sort_by_key(|n| n.sort_order);
        roots
    }

    pub fn navigate_to_root_public(&self) -> (OutgoingMessage, Option<i64>) {
        self.navigate_to_root()
    }
}
