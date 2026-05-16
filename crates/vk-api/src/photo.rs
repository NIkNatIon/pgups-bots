use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UploadServerResponse {
    pub response: Option<UploadServer>,
}

#[derive(Debug, Deserialize)]
pub struct UploadServer {
    pub upload_url: String,
}

#[derive(Debug, Deserialize)]
pub struct UploadResult {
    pub photo: String,
    pub server: i64,
    pub hash: String,
}

#[derive(Debug, Deserialize)]
pub struct SavePhotoResponse {
    pub response: Option<Vec<SavedPhoto>>,
}

#[derive(Debug, Deserialize)]
pub struct SavedPhoto {
    pub id: i64,
    pub owner_id: i64,
}

impl SavedPhoto {
    pub fn to_attachment(&self) -> String {
        format!("photo{}_{}", self.owner_id, self.id)
    }
}

pub fn build_multipart_body(file_bytes: &[u8], filename: &str) -> (String, Vec<u8>) {
    let boundary = "----WasselBotBoundary7MA4YWxkTrZu0gW";
    let content_type = format!("multipart/form-data; boundary={}", boundary);

    let mut body = Vec::new();

    body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
    body.extend_from_slice(
        format!(
            "Content-Disposition: form-data; name=\"photo\"; filename=\"{}\"\r\n",
            filename
        )
        .as_bytes(),
    );
    body.extend_from_slice(b"Content-Type: image/jpeg\r\n");
    body.extend_from_slice(b"\r\n");

    body.extend_from_slice(file_bytes);

    body.extend_from_slice(format!("\r\n--{}--\r\n", boundary).as_bytes());

    (content_type, body)
}
