use std::io::BufRead;
use std::io::BufReader;
use std::str;
use std::net::TcpListener;
use std::net::TcpStream;
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
}
