use core::fmt;
use std::collections::HashMap;
use std::error::Error;
use std::net::TcpStream;
use std::str;

use std::io::BufRead;
use std::io::BufReader;

use serde_json::Value;

pub struct Request {
    pub method: String,
    pub uri: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Value>,
    pub buffer: Vec<u8>,
    pub buffer_string: String,
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let body_json = match &self.body {
            Some(value) => serde_json::to_string_pretty(&value).unwrap(),
            None => String::from(""),
        };
        write!(
            f,
            "{}",
            format!(
                "{}\n{}\n{}\n{:#?}\n{}",
                self.method, self.uri, self.version, self.headers, body_json
            )
        )
    }
}

impl Request {
    pub fn parse(mut stream: &TcpStream) -> Result<Request, Box<dyn Error>> {
        let mut reader = BufReader::new(&mut stream);
        // buffer contains all the string text of http request
        let buffer: Vec<u8> = reader.fill_buf()?.to_vec();

        // empty header placeholder
        let mut headers = [httparse::EMPTY_HEADER; 16];
        // init request object httparse
        let mut req = httparse::Request::new(&mut headers);

        let offset = match req.parse(&buffer)? {
            httparse::Status::Complete(offset) => offset,
            httparse::Status::Partial => return Err("Uncomplete request".into()),
        };

        // http text request string
        let buffer_string = str::from_utf8(&buffer)?;

        let method = req.method.ok_or("Method not found")?;
        let uri = req.path.ok_or("URI not found")?.to_string();
        let version = req.version.ok_or("Version not found")?.to_string();

        let mut headers_map = HashMap::new();
        for header in req.headers.iter() {
            let name = header.name.to_string();
            let value = std::str::from_utf8(header.value)?.to_string();
            headers_map.insert(name, value);
        }

        let body = if offset < buffer.len() {
            let body_string = String::from_utf8(buffer[offset..].to_vec())?;
            let body_json: Value = serde_json::from_str(&body_string)?;
            Some(body_json)
        } else {
            None
        };

        Ok(Request {
            method: method.to_string(),
            uri,
            version,
            headers: headers_map,
            body,
            buffer: buffer.clone(),
            buffer_string: buffer_string.to_string(),
        })
    }
}
