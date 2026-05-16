use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Update {
    pub update_id: i64,
    pub message: Option<Message>,
    pub callback_query: Option<CallbackQuery>,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub message_id: i64,
    pub from: Option<User>,
    pub chat: Chat,
    pub text: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub message: Option<Message>,
    pub data: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: i64,
    pub first_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Chat {
    pub id: i64,
}

impl Update {
    pub fn parse(body: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(body)
    }

    pub fn extract(&self) -> Option<(i64, Option<&str>, Option<&str>, Option<&str>)> {
        if let Some(cb) = &self.callback_query {
            let chat_id = cb.message.as_ref()?.chat.id;
            let payload = cb.data.as_deref();
            return Some((chat_id, payload, None, Some(&cb.id)));
        }

        if let Some(msg) = &self.message {
            let chat_id = msg.chat.id;
            let text = msg.text.as_deref();
            return Some((chat_id, None, text, None));
        }

        None
    }
}
