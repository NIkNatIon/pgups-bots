use bot_core::messenger::Button;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct VkKeyboard {
    pub one_time: bool,
    pub buttons: Vec<Vec<VkButton>>,
}

#[derive(Debug, Serialize)]
pub struct VkButton {
    pub action: VkButtonAction,
}

#[derive(Debug, Serialize)]
pub struct VkButtonAction {
    #[serde(rename = "type")]
    pub action_type: String,
    pub label: String,
    pub payload: String,
}

impl From<&[Button]> for VkKeyboard {
    fn from(buttons: &[Button]) -> Self {
        let rows = buttons
            .iter()
            .take(10)
            .map(|btn| {
                vec![VkButton {
                    action: VkButtonAction {
                        action_type: "text".to_string(),
                        label: btn.label.clone(),
                        payload: format!("\"{}\"", btn.payload),
                    },
                }]
            })
            .collect();

        Self {
            one_time: false,
            buttons: rows,
        }
    }
}

impl VkKeyboard {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}
