pub mod db;
pub mod menu;
pub mod messenger;
pub mod schedule;
pub mod texts;

#[cfg(test)]
mod tests;

use menu::MenuNode;
use messenger::{BotResponse, Button, OutgoingMessage};
use schedule::{
    current_parity, current_weekday, next_week_parity, parity_ru, tomorrow_weekday, weekday_ru,
};
use texts::Texts;

pub struct BotHandler {
    menu_nodes: Vec<MenuNode>,
    texts: Texts,
}

impl BotHandler {
    pub fn new(menu_nodes: Vec<MenuNode>, texts: Texts) -> Self {
        Self { menu_nodes, texts }
    }

    pub fn menu_nodes_ref(&self) -> &[MenuNode] {
        &self.menu_nodes
    }

    pub fn texts_ref(&self) -> &Texts {
        &self.texts
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
                "schedule_today" => {
                    if let Some(group) = student_group {
                        return BotResponse::ScheduleRequest {
                            group: group.to_string(),
                            weekday: current_weekday().to_string(),
                            parity: current_parity().to_string(),
                            new_node_id: user_node_id,
                        };
                    } else {
                        return BotResponse::AskGroup {
                            new_node_id: user_node_id,
                        };
                    }
                }
                "schedule_tomorrow" => {
                    if let Some(group) = student_group {
                        let tomorrow = tomorrow_weekday();
                        let parity = if tomorrow == "Monday" {
                            next_week_parity()
                        } else {
                            current_parity()
                        };
                        return BotResponse::ScheduleRequest {
                            group: group.to_string(),
                            weekday: tomorrow.to_string(),
                            parity: parity.to_string(),
                            new_node_id: user_node_id,
                        };
                    } else {
                        return BotResponse::AskGroup {
                            new_node_id: user_node_id,
                        };
                    }
                }
                "schedule_this_week" => {
                    if let Some(group) = student_group {
                        return BotResponse::ScheduleWeekRequest {
                            group: group.to_string(),
                            parity: current_parity().to_string(),
                            new_node_id: user_node_id,
                        };
                    } else {
                        return BotResponse::AskGroup {
                            new_node_id: user_node_id,
                        };
                    }
                }
                "schedule_next_week" => {
                    if let Some(group) = student_group {
                        return BotResponse::ScheduleWeekRequest {
                            group: group.to_string(),
                            parity: next_week_parity().to_string(),
                            new_node_id: user_node_id,
                        };
                    } else {
                        return BotResponse::AskGroup {
                            new_node_id: user_node_id,
                        };
                    }
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
                let today = weekday_ru(current_weekday());
                let parity = parity_ru(current_parity());

                OutgoingMessage {
                    text: self.texts.format(
                        "msg.schedule_header",
                        &[("group", group), ("weekday", today), ("parity", parity)],
                    ),
                    buttons: vec![
                        Button {
                            label: self.texts.get("btn.schedule_today").into(),
                            payload: "schedule_today".into(),
                        },
                        Button {
                            label: self.texts.get("btn.schedule_tomorrow").into(),
                            payload: "schedule_tomorrow".into(),
                        },
                        Button {
                            label: self.texts.get("btn.schedule_this_week").into(),
                            payload: "schedule_this_week".into(),
                        },
                        Button {
                            label: self.texts.get("btn.schedule_next_week").into(),
                            payload: "schedule_next_week".into(),
                        },
                        Button {
                            label: self.texts.get("btn.schedule_change_group").into(),
                            payload: "schedule_change_group".into(),
                        },
                        Button {
                            label: self.texts.get("btn.back").into(),
                            payload: self.find_schedule_parent_id(),
                        },
                        Button {
                            label: self.texts.get("btn.home").into(),
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
                text: self.texts.get("msg.ask_group").into(),
                buttons: vec![
                    Button {
                        label: self.texts.get("btn.back").into(),
                        payload: self.find_schedule_parent_id(),
                    },
                    Button {
                        label: self.texts.get("btn.home").into(),
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

        let children = self.get_children(node.id);
        let mut buttons: Vec<Button> = children
            .iter()
            .map(|child| Button {
                label: child.title.clone(),
                payload: child.id.to_string(),
            })
            .collect();

        if let Some(parent_id) = node.parent_id {
            buttons.push(Button {
                label: self.texts.get("btn.back").to_string(),
                payload: parent_id.to_string(),
            });

            let parent = self.menu_nodes.iter().find(|n| n.id == parent_id);
            if parent.is_some_and(|p| p.parent_id.is_some())
                && let Some(root) = self.get_roots().first()
            {
                buttons.push(Button {
                    label: self.texts.get("btn.home").to_string(),
                    payload: root.id.to_string(),
                });
            }
        }

        let msg = OutgoingMessage {
            text: node.content.clone().unwrap_or_else(|| node.title.clone()),
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
            .map(|node| Button {
                label: node.title.clone(),
                payload: node.id.to_string(),
            })
            .collect();

        let msg = OutgoingMessage {
            text: self.texts.get("msg.select_section").to_string(),
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
            .find(|n| n.title.to_lowercase() == text)
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
}
