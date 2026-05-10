pub mod menu;
pub mod messenger;
pub mod db;

#[cfg(test)]
mod tests;

use menu::MenuNode;
use messenger::{OutgoingMessage, Button};

pub struct BotHandler {
    menu_nodes: Vec<MenuNode>,
}

impl BotHandler {
    pub fn new(menu_nodes: Vec<MenuNode>) -> Self {
        Self { menu_nodes }
    }

    pub fn handle_message(
        &self,
        user_node_id: Option<i64>,
        payload: Option<&str>,
        text: Option<&str>,
    ) -> (OutgoingMessage, Option<i64>) {
        let target_node_id = payload
            .and_then(|p| p.parse::<i64>().ok())
            .or_else(|| self.find_node_by_text(user_node_id, text));

        match target_node_id {
            Some(node_id) => self.navigate_to(node_id),
            None => self.navigate_to_root(),
        }
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
                label: "⬅ Назад".to_string(),
                payload: parent_id.to_string(),
            });

            let parent = self.menu_nodes.iter().find(|n| n.id == parent_id);
            if parent.is_some_and(|p| p.parent_id.is_some()) {
                if let Some(root) = self.get_roots().first() {
                    buttons.push(Button {
                        label: "🏠 В начало".to_string(),
                        payload: root.id.to_string(),
                    });
                }
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
            text: "Выберите раздел:".to_string(),
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