use crate::http_client::common::HttpClientResponse;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize)]
pub struct CheckCaptchaRequest<'a> {
    pub secret: &'a str,
    pub response: &'a str,
    pub remoteip: &'a str,
}

pub async fn check_captcha(
    request: CheckCaptchaRequest<'_>,
) -> Result<HttpClientResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let resp = client
        .post("https://challenges.cloudflare.com/turnstile/v0/siteverify")
        .header("accept", "application/json")
        .header("content-type", "application/json")
        .json(&request)
        .send()
        .await;

    match resp {
        Ok(res) => {
            let status_code = res.status();
            match res.json::<HashMap<String, String>>().await {
                Ok(j) => {
                    let ret = HttpClientResponse {
                        status: status_code,
                        body: j,
                    };
                    return Ok(ret);
                }
                Err(err) => {
                    return Err(Box::new(err));
                }
            }
        }
        Err(err) => {
            return Err(Box::new(err));
        }
    }
}
