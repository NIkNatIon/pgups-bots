use crate::keyboard::TgInlineKeyboard;
use bot_core::messenger::Button;

pub struct TgSender {
    pub bot_token: String,
}

impl TgSender {
    pub fn new(bot_token: String) -> Self {
        Self { bot_token }
    }

    pub fn build_send_url(&self, chat_id: i64, text: &str, buttons: &[Button]) -> String {
        let encoded_text = urlencode(text);

        let mut url = format!(
            "https://api.telegram.org/bot{}/sendMessage?\
             chat_id={chat_id}\
             &text={encoded_text}\
             &parse_mode=HTML",
            self.bot_token
        );

        if !buttons.is_empty() {
            let keyboard = TgInlineKeyboard::from_buttons(buttons);
            url.push_str("&reply_markup=");
            url.push_str(&urlencode(&keyboard.to_json()));
        }

        url
    }

    pub fn build_answer_callback_url(&self, callback_query_id: &str) -> String {
        format!(
            "https://api.telegram.org/bot{}/answerCallbackQuery?\
             callback_query_id={}",
            self.bot_token, callback_query_id
        )
    }

    pub fn build_set_webhook_url(&self, webhook_url: &str) -> String {
        format!(
            "https://api.telegram.org/bot{}/setWebhook?\
             url={}",
            self.bot_token,
            urlencode(webhook_url)
        )
    }

    pub fn send_photo_url(&self) -> String {
        format!("https://api.telegram.org/bot{}/sendPhoto", self.bot_token)
    }

    pub fn build_send_photo_body(
        &self,
        chat_id: i64,
        photo_bytes: &[u8],
        caption: &str,
        buttons: &[Button],
    ) -> (String, Vec<u8>) {
        let boundary = "----WasselTgBotBoundary";
        let content_type = format!("multipart/form-data; boundary={}", boundary);

        let mut body = Vec::new();

        body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
        body.extend_from_slice(b"Content-Disposition: form-data; name=\"chat_id\"\r\n\r\n");
        body.extend_from_slice(chat_id.to_string().as_bytes());
        body.extend_from_slice(b"\r\n");

        if !caption.is_empty() {
            body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
            body.extend_from_slice(b"Content-Disposition: form-data; name=\"caption\"\r\n\r\n");
            body.extend_from_slice(caption.as_bytes());
            body.extend_from_slice(b"\r\n");
        }

        if !buttons.is_empty() {
            let keyboard = TgInlineKeyboard::from_buttons(buttons);
            body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
            body.extend_from_slice(
                b"Content-Disposition: form-data; name=\"reply_markup\"\r\n\r\n",
            );
            body.extend_from_slice(keyboard.to_json().as_bytes());
            body.extend_from_slice(b"\r\n");
        }

        body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
        body.extend_from_slice(
            b"Content-Disposition: form-data; name=\"photo\"; filename=\"photo.jpg\"\r\n",
        );
        body.extend_from_slice(b"Content-Type: image/jpeg\r\n\r\n");
        body.extend_from_slice(photo_bytes);
        body.extend_from_slice(format!("\r\n--{}--\r\n", boundary).as_bytes());

        (content_type, body)
    }
}

fn urlencode(s: &str) -> String {
    let mut result = String::new();
    for byte in s.as_bytes() {
        match *byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(*byte as char);
            }
            _ => {
                result.push_str(&format!("%{:02X}", byte));
            }
        }
    }
    result
}
