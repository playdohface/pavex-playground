use pavex::http::header::{ToStrError, USER_AGENT};
use pavex::request::RequestHead;
use pavex::response::Response;
pub enum UserAgent {
    Unknown,
    Known(String),
}

impl UserAgent {
    pub fn extract(header: &RequestHead) -> Result<Self, ToStrError> {
        let Some(useragent) = header.headers.get(USER_AGENT) else {
            return Ok(Self::Unknown);
        };
        useragent.to_str().map(|s| Self::Known(s.into()))
    }
}

pub fn invalid_user_agent(_e: &ToStrError) -> Response {
    Response::bad_request().set_typed_body("User-Agent must be valid.")
}
