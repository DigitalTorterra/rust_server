extern crate httparse;

use std::io::prelude::*;
use std::net::TcpStream;
use std::fs;


pub fn handle_connection(mut stream: TcpStream) {
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


pub struct Config {
    pub name: String,
    pub port: String,
    pub root: String,
    pub help: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let name = args[0].clone();
        let mut port = String::from("7878");
        let mut root = String::from(".");
        let mut help = false;

        for i in 1..args.len() {
            if args[i] == "-h" {
                help = true;
            }

            else if args[i] == "-p" {
                if i != args.len() - 1 {
                    port = args[i + 1].clone();
                }
            }

            else if args[i] == "-r" {
                if i != args.len() - 1 {
                    root = args[i + 1].clone();
                }
            }
        }


        Ok(Config { name, port, root, help })
    }

    pub fn print_help(&self) {
        let mut help_message = "Usage: ".to_string();
        help_message.push_str(&self.name);
        help_message.push_str(&String::from(" [hpr]
Options:
    -h  Display help message
    -p  Port to listen on
    -r  Root directory"));

        println!("{}", help_message);
    }
}
