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

pub struct UserInfo {
    pub lang: String,
    pub role: Option<String>,
    pub student_group: Option<String>,
    pub onboarded: bool,
}

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

    pub fn navigate_to_root_public(&self) -> (OutgoingMessage, Option<i64>) {
        self.navigate_to_root()
    }

    pub fn handle_message(
        &self,
        user_node_id: Option<i64>,
        payload: Option<&str>,
        text: Option<&str>,
        user_info: &UserInfo,
    ) -> BotResponse {
        if !user_info.onboarded {
            return self.handle_onboarding(payload, user_info);
        }

        if let Some(t) = text
            && t == "/start"
        {
            return self.build_main_menu(user_info);
        }

        if let Some(p) = payload {
            match p {
                "main_menu" => return self.build_main_menu(user_info),
                "open_info" => return self.open_info_for_role(user_info),
                "open_settings" => return BotResponse::Settings,
                "change_language" => return BotResponse::LanguageSelect,
                "change_role" => return BotResponse::OnboardingAskRole,
                "change_group" => {
                    return BotResponse::AskGroup {
                        new_node_id: user_node_id,
                    };
                }
                "settings_change_group" => return BotResponse::SettingsChangeGroup,
                "lang_ru" | "lang_en" | "lang_zh" => {
                    let lang = p.strip_prefix("lang_").unwrap_or("ru");
                    return BotResponse::LanguageChanged {
                        lang: lang.to_string(),
                    };
                }
                "role_applicant" | "role_student" | "role_teacher" | "role_guest" => {
                    let role = p.strip_prefix("role_").unwrap_or("guest");
                    return BotResponse::RoleChanged {
                        role: role.to_string(),
                    };
                }
                "schedule_today" => {
                    return if let Some(group) = &user_info.student_group {
                        BotResponse::ScheduleRequest {
                            group: group.clone(),
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
                    return if let Some(group) = &user_info.student_group {
                        let tomorrow = tomorrow_weekday();
                        let parity = if tomorrow == "Monday" {
                            next_week_parity()
                        } else {
                            current_parity()
                        };
                        BotResponse::ScheduleRequest {
                            group: group.clone(),
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
                    return if let Some(group) = &user_info.student_group {
                        BotResponse::ScheduleWeekRequest {
                            group: group.clone(),
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
                    return if let Some(group) = &user_info.student_group {
                        BotResponse::ScheduleWeekRequest {
                            group: group.clone(),
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
            if user_info.student_group.is_some() {
                return BotResponse::Message(
                    self.schedule_menu(user_info.student_group.as_deref()),
                    Some(node_id),
                );
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
                    return BotResponse::Message(
                        self.schedule_menu(user_info.student_group.as_deref()),
                        Some(node_id),
                    );
                }
                let (msg, nid) = self.navigate_to(node_id);
                BotResponse::Message(msg, nid)
            }
            None => self.build_main_menu(user_info),
        }
    }

    fn handle_onboarding(&self, payload: Option<&str>, user_info: &UserInfo) -> BotResponse {
        if let Some(p) = payload {
            if p.starts_with("lang_") {
                let lang = p.strip_prefix("lang_").unwrap_or("ru");
                return BotResponse::LanguageChanged {
                    lang: lang.to_string(),
                };
            }
            if p.starts_with("role_") {
                let role = p.strip_prefix("role_").unwrap_or("guest");
                return BotResponse::RoleChanged {
                    role: role.to_string(),
                };
            }
            if p == "skip_group" {
                return BotResponse::OnboardingAskGroup;
            }
        }

        if user_info.lang.is_empty() {
            return BotResponse::OnboardingAskLang;
        }

        if user_info.role.is_none() {
            return BotResponse::OnboardingAskRole;
        }

        if user_info.role.as_deref() == Some("student") && user_info.student_group.is_none() {
            return BotResponse::OnboardingAskGroup;
        }

        BotResponse::OnboardingAskLang
    }

    pub fn build_main_menu(&self, user_info: &UserInfo) -> BotResponse {
        let mut buttons = vec![
            Button {
                label: self.i18n.get("menu-info-title"),
                payload: "open_info".into(),
            },
            Button {
                label: self.i18n.get("menu-about-title"),
                payload: self
                    .menu_nodes
                    .iter()
                    .find(|n| n.slug == "about")
                    .map(|n| n.id.to_string())
                    .unwrap_or("3".into()),
            },
        ];

        if user_info
            .student_group
            .as_deref()
            .is_some_and(|g| !g.is_empty())
        {
            buttons.insert(
                0,
                Button {
                    label: self.i18n.get("menu-schedule-title"),
                    payload: self
                        .menu_nodes
                        .iter()
                        .find(|n| n.slug == "schedule")
                        .map(|n| n.id.to_string())
                        .unwrap_or("4".into()),
                },
            );
        }

        buttons.push(Button {
            label: self.i18n.get("btn-settings"),
            payload: "open_settings".into(),
        });

        BotResponse::Message(
            OutgoingMessage {
                text: self.i18n.get("menu-start-content"),
                buttons,
                image_url: None,
            },
            self.menu_nodes
                .iter()
                .find(|n| n.slug == "start")
                .map(|n| n.id),
        )
    }

    fn open_info_for_role(&self, user_info: &UserInfo) -> BotResponse {
        let role = user_info.role.as_deref().unwrap_or("guest");

        let slug = match role {
            "applicant" => "abit",
            "student" => "stud",
            "teacher" => "prof",
            "guest" => {
                if let Some(info_node) = self.menu_nodes.iter().find(|n| n.slug == "info") {
                    let (msg, nid) = self.navigate_to(info_node.id);
                    return BotResponse::Message(msg, nid);
                }
                return self.build_main_menu(user_info);
            }
            _ => {
                if let Some(info_node) = self.menu_nodes.iter().find(|n| n.slug == "info") {
                    let (msg, nid) = self.navigate_to(info_node.id);
                    return BotResponse::Message(msg, nid);
                }
                return self.build_main_menu(user_info);
            }
        };

        if let Some(node) = self.menu_nodes.iter().find(|n| n.slug == slug) {
            let (msg, nid) = self.navigate_to(node.id);
            BotResponse::Message(msg, nid)
        } else {
            self.build_main_menu(user_info)
        }
    }

    pub fn build_settings(&self, user_info: &UserInfo) -> OutgoingMessage {
        let lang_display = match user_info.lang.as_str() {
            "ru" => "🇷🇺 Русский",
            "en" => "🇬🇧 English",
            "zh" => "🇨🇳 中文",
            _ => &user_info.lang,
        };

        let role_key = format!("role-{}", user_info.role.as_deref().unwrap_or("guest"));
        let role_display = self.i18n.get(&role_key);

        let group_display = user_info.student_group.as_deref().unwrap_or("—");

        let text = format!(
            "{}\n\n{}\n{}\n{}",
            self.i18n.get("msg-settings"),
            self.i18n
                .format("msg-settings-lang", &[("lang", lang_display)]),
            self.i18n
                .format("msg-settings-role", &[("role", &role_display)]),
            self.i18n
                .format("msg-settings-group", &[("group", group_display)]),
        );

        let buttons = vec![
            Button {
                label: self.i18n.get("btn-settings-change-lang"),
                payload: "change_language".into(),
            },
            Button {
                label: self.i18n.get("btn-settings-change-role"),
                payload: "change_role".into(),
            },
            Button {
                label: self.i18n.get("btn-settings-change-group"),
                payload: "settings_change_group".into(),
            },
            Button {
                label: self.i18n.get("btn-back"),
                payload: "main_menu".into(),
            },
        ];

        OutgoingMessage {
            text,
            buttons,
            image_url: None,
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
                            payload: "main_menu".into(),
                        },
                    ],
                    image_url: None,
                }
            }
            None => OutgoingMessage {
                text: self.i18n.get("msg-ask-group"),
                buttons: vec![Button {
                    label: self.i18n.get("btn-back"),
                    payload: "main_menu".into(),
                }],
                image_url: None,
            },
        }
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
            let is_root_child = self
                .menu_nodes
                .iter()
                .find(|n| n.id == parent_id)
                .is_some_and(|p| p.parent_id.is_none());
            let back_payload =
                if is_root_child || matches!(node.slug.as_str(), "abit" | "stud" | "prof") {
                    "main_menu".to_string()
                } else {
                    parent_id.to_string()
                };
            buttons.push(Button {
                label: self.i18n.get("btn-back"),
                payload: back_payload,
            });
            if !is_root_child && !matches!(node.slug.as_str(), "abit" | "stud" | "prof") {
                let parent = self.menu_nodes.iter().find(|n| n.id == parent_id);
                if parent.is_some_and(|p| p.parent_id.is_some()) {
                    buttons.push(Button {
                        label: self.i18n.get("btn-home"),
                        payload: "main_menu".into(),
                    });
                }
            }
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
}
