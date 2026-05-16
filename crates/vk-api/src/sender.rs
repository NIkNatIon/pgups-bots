use crate::keyboard::VkKeyboard;
use bot_core::messenger::Button;

pub struct VkSender {
    pub access_token: String,
    pub api_version: String,
}

impl VkSender {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            api_version: "5.199".to_string(),
        }
    }

    pub fn build_send_url(
        &self,
        peer_id: i64,
        text: &str,
        buttons: &[Button],
        attachment: Option<&str>,
    ) -> String {
        let encoded_text = urlencode(text);

        let mut url = format!(
            "https://api.vk.com/method/messages.send?\
             peer_id={peer_id}\
             &random_id=0\
             &message={encoded_text}\
             &access_token={}\
             &v={}",
            self.access_token, self.api_version
        );

        if !buttons.is_empty() {
            let keyboard = VkKeyboard::from(buttons);
            url.push_str("&keyboard=");
            url.push_str(&urlencode(&keyboard.to_json()));
        }

        if let Some(att) = attachment {
            url.push_str("&attachment=");
            url.push_str(att);
        }

        url
    }

    pub fn build_get_upload_server_url(&self, peer_id: i64) -> String {
        format!(
            "https://api.vk.com/method/photos.getMessagesUploadServer?\
             peer_id={peer_id}\
             &access_token={}\
             &v={}",
            self.access_token, self.api_version
        )
    }

    pub fn build_save_photo_url(&self, photo: &str, server: i64, hash: &str) -> String {
        format!(
            "https://api.vk.com/method/photos.saveMessagesPhoto?\
             photo={}\
             &server={server}\
             &hash={hash}\
             &access_token={}\
             &v={}",
            urlencode(photo),
            self.access_token,
            self.api_version
        )
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
