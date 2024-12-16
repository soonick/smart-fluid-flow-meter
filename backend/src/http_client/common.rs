use reqwest::StatusCode;
use std::collections::HashMap;

pub struct HttpClientResponse {
    pub status: StatusCode,
    pub body: HashMap<String, String>,
}
