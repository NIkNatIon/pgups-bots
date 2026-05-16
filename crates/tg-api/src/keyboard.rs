use bot_core::messenger::Button;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TgInlineKeyboard {
    pub inline_keyboard: Vec<Vec<TgInlineButton>>,
}

#[derive(Debug, Serialize)]
pub struct TgInlineButton {
    pub text: String,
    pub callback_data: String,
}

impl From<&[Button]> for TgInlineKeyboard {
    fn from(buttons: &[Button]) -> Self {
        let rows = buttons
            .iter()
            .map(|btn| {
                vec![TgInlineButton {
                    text: btn.label.clone(),
                    callback_data: btn.payload.clone(),
                }]
            })
            .collect();

        Self {
            inline_keyboard: rows,
        }
    }
}

impl TgInlineKeyboard {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}
