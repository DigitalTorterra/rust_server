extern crate threadpool;
extern crate httparse;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use threadpool::ThreadPool;
//use httparse;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    let n_workers = 4;
    let pool = ThreadPool::new(n_workers);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}


fn handle_connection(mut stream: TcpStream) {
    // Allocate buffers and request
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);
    let res = req.parse(&buffer).unwrap();

    if res.is_partial() {
        return;
    }

    // req fields: method, path, version, headers
    let (status_line, filename) = if req.path == Some("/") {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
