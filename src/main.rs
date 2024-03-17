use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::io::Error as IoError;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

mod http;

use http::request::Request;

fn main() -> Result<(), IoError> {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let test = json!({"test":"Test"});
        let _ = serde_json::to_string(&test);

        // thread::sleep(Duration::from_millis(5000));
        let mut response_header: HashMap<String, String> = HashMap::new();
        response_header.insert("Content-type".into(), "application/json".into());
        response_header.insert("Server".into(), "localhost".into());

        let request = match Request::parse(&mut stream) {
            Ok(request) => request,
            Err(err) => {
                handle_parse_error(
                    &mut stream,
                    response_header,
                    json!({
                        "success": false,
                        "error": format!("{}", err)
                    }),
                )?;
                continue;
            }
        };
        println!("{}", request.buffer_string);
        let response = format!(
            "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}",
            200,
            "OK",
            response_header
                .iter()
                .map(|(key, value)| format!("{}: {}", key, value))
                .collect::<Vec<_>>()
                .join("\r\n"),
            json!({"success": true})
        );

        println!("{}\n=====", response);

        stream.write_all(response.as_bytes()).unwrap();
    }
    Ok(())
}

fn handle_parse_error(
    stream: &mut TcpStream,
    header: HashMap<String, String>,
    body: Value,
) -> Result<(), IoError> {
    let response = format!(
        "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}",
        400,
        "Bad Request",
        header
            .iter()
            .map(|(key, value)| format!("{}: {}", key, value))
            .collect::<Vec<_>>()
            .join("\r\n"),
        body
    );
    println!("{}", response);
    stream.write_all(response.as_bytes())?;
    Ok(())
}
