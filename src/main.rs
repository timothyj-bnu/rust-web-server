use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
        thread::sleep(Duration::from_millis(5000));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut reader = BufReader::new(&mut stream);
    let buffer: Vec<u8> = match reader.fill_buf() {
        Ok(buf) => buf.to_vec(),
        Err(err) => {
            println!("Error reading buffer: {}", err);
            return;
        }
    };

    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);

    println!("{:?}", buffer);

    let res = match req.parse(&buffer).unwrap() {
        httparse::Status::Complete(amt) => amt,
        httparse::Status::Partial => {
            return;
        }
    };

    println!("{:?}", res);

    let string = match str::from_utf8(&buffer) {
        Ok(string) => string,
        Err(e) => {
            println!("Error reading UTF-8 sequence: {}", e);
            return;
        }
    };

    println!("Request:\n{}", string);

    let method = match req.method.ok_or("Method not found") {
        Ok(method) => method,
        Err(_) => {
            return;
        }
    };
    let uri = req.path.ok_or("URI not found").unwrap().to_string();
    let version = req.version.ok_or("Version not found").unwrap().to_string();

    let mut headers_map = HashMap::new();
    for header in req.headers.iter() {
        let name = header.name.to_string();
        let value = std::str::from_utf8(header.value).unwrap().to_string();
        headers_map.insert(name, value);
    }

    let body = if res < buffer.len() {
        Some(String::from_utf8(buffer[res..].to_vec()))
    } else {
        None
    };


    println!("{}\n{}\n{}\n{:#?}\n{}", method, uri, version, headers_map, body.unwrap().unwrap());



}
