use std::collections::HashMap;

use chrono::Utc;
use serde_json::Value;

pub struct Response {
    pub http_response_code: i32,
    pub http_response_text: String,
    pub headers: HashMap<String, String>,
    pub body: Value,
}

impl Response {
    pub fn new(
        http_response_code: i32,
        http_response_text: String,
        mut headers: HashMap<String, String>,
        body: Value,
    ) -> Response {
        headers.insert("Content-type".into(), "application/json".into());
        headers.insert("Server".into(), "localhost".into());
        let content_length = serde_json::to_string(&body).unwrap().as_bytes().len();
        headers.insert("Content-length".into(), format!("{}", content_length));
        let now = Utc::now();
        let formatted = now.format("%a, %d %b %Y %H:%M:%S GMT");
        headers.insert("Date".into(), format!("{}", formatted));
        Response {
            http_response_code,
            http_response_text,
            headers,
            body,
        }
    }

    pub fn as_vec_bytes(&self) -> Vec<u8> {
        let response_string = format!(
            "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}",
            self.http_response_code,
            self.http_response_text,
            self.headers
                .iter()
                .map(|(key, value)| format!("{}: {}", key, value))
                .collect::<Vec<_>>()
                .join("\r\n"),
            self.body
        );

        // println!("{}", response_string);

        response_string.into_bytes()
    }
}
