use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

use std::error::Error;
use std::thread;
use std::time::Duration;

mod http;
mod multithread;

use http::request::Request;
use http::response::Response;
use multithread::thread_pool::ThreadPool;

fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(2);

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        pool.execute(move || {
            let response_header: HashMap<String, String> = HashMap::new();

            let request = match Request::parse(&mut stream) {
                Ok(request) => request,
                Err(err) => {
                    match handle_parse_error(
                        &mut stream,
                        response_header,
                        json!({
                            "success": false,
                            "error": format!("{}", err)
                        }),
                    ) {
                        Ok(_) => return,
                        Err(_) => return,
                    }
                }
            };
            // println!("{}", request.buffer_string);
            let response = Response::new(
                200,
                "OK".into(),
                response_header,
                json!({"success":true, "text":"❤️"}),
            );

            thread::sleep(Duration::from_millis(5000));

            stream
                .write_all(response.as_vec_bytes().as_slice())
                .unwrap();
        });
    }
    Ok(())
}

fn handle_parse_error(
    stream: &mut TcpStream,
    header: HashMap<String, String>,
    body: Value,
) -> Result<(), Box<dyn Error>> {
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
    stream.write_all(response.as_bytes())?;
    Ok(())
}
