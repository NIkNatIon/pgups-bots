#[derive(Debug, Clone)]
pub struct Button {
    pub label: String,
    pub payload: String,
}

#[derive(Debug, Clone)]
pub struct OutgoingMessage {
    pub text: String,
    pub buttons: Vec<Button>,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone)]
pub enum BotResponse {
    Message(OutgoingMessage, Option<i64>),
    ScheduleRequest {
        group: String,
        weekday: String,
        parity: String,
        new_node_id: Option<i64>,
    },
    ScheduleWeekRequest {
        group: String,
        parity: String,
        new_node_id: Option<i64>,
    },
    AskGroup {
        new_node_id: Option<i64>,
    },
}

pub trait Messenger {
    type Error: core::fmt::Debug;
    fn send_message(&self, user_id: i64, message: &OutgoingMessage) -> Result<(), Self::Error>;
}
