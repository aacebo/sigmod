use crate::client::ClientExtension;

pub struct BearerExtension(String);

impl ClientExtension for BearerExtension {
    fn request(&self, mut req: reqwest::Request) -> reqwest::Request {
        req.headers_mut().insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", self.0)).unwrap(),
        );

        req
    }
}
