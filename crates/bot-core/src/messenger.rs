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

pub trait Messenger {
    type Error: core::fmt::Debug;
    fn send_message(&self, user_id: i64, message: &OutgoingMessage) -> Result<(), Self::Error>;
}