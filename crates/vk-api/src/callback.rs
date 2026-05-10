use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CallbackEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub object: Option<serde_json::Value>,
    pub secret: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MessageObject {
    pub message: Message,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub from_id: i64,
    pub peer_id: i64,
    pub text: String,
    pub payload: Option<String>,
}

impl CallbackEvent {
    pub fn parse(body: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(body)
    }

    pub fn into_message(self) -> Option<Message> {
        if self.event_type != "message_new" {
            return None;
        }
        let obj: MessageObject = serde_json::from_value(self.object?).ok()?;
        Some(obj.message)
    }
}