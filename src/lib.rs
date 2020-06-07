extern crate httparse;
extern crate serde_json;

use std::io::prelude::*;
use std::io::BufReader;
use std::io::ErrorKind;
use std::net::TcpStream;
use std::sync::Arc;
use std::fs;
use std::fs::File;
use log::{ debug, info };
use serde_json::{ Value, Error, Map };


pub fn handle_connection(mut stream: TcpStream, root: Arc<String>, site_structure: Arc<Value>) {
    // Allocate buffers and request
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);
    let res = req.parse(&buffer).unwrap();

    if res.is_partial() {
        debug!("Received a partial request");
        return;
    }

    info!("Parsed HTTP request");
    let site_clone = Arc::clone(&site_structure);


    let response = match get_request_path(&req, root, site_structure) {
        Ok(file) => handle_request(file, &req),
        Err(error) => handle_error(error, site_clone),
    };


    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}


pub fn parse_site_structure(root: &String) -> Result<Value, serde_json::Error> {
    // Open the structure file
    let path = format!("{}/structure.json", root);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let out = serde_json::from_reader(reader)?;

    Ok(out)
}

// Parse the URI and request, return name of file to send back
pub fn get_request_path(req: &httparse::Request, root: Arc<String>, site_structure: Arc<Value>) -> Result<String, String> {
    let split = req.path.unwrap().split("/");
    let mut curr = &site_structure["/"];

    debug!("Requested Path: {}", req.path.unwrap());

    // We start iteration at the root "/"
    for s in split {
        if s.is_empty() {
            continue;
        }

        debug!("Current Element: {}", s);

        if curr.as_object().unwrap().contains_key("children") {
            curr = &curr["children"];
        }

        else {
            debug!("No children found");
            return Err("404".to_string());
        }

        if curr.as_object().unwrap().contains_key(&s.to_string()) {
            curr = &curr[s];
        }

        else {
            debug!("Could not find the child for {}", s);
            return Err("404".to_string());
        }
    }

    let target = curr["file"].as_str().unwrap();
    let out = format!("{}/{}", root, target);
    
    debug!("Target: {}", target);
    debug!("Parsed Path: {}", out);
    Ok(out)
}


pub fn handle_error(mut code: String, site_structure: Arc<Value>) -> String {
    debug!("Handling {} error", code);

    let errors = &site_structure["error"];

    let mut filename: String;
    if errors.as_object().unwrap().contains_key(&code) {
        filename = errors[&code].as_str().unwrap().to_string();
    }

    else {
        code = "500".to_string();
        filename = errors[&code].as_str().unwrap().to_string();
    }

    debug!("Accessing {}", filename);

    let status_msg = match code.as_str() {
        "404" => "NOT FOUND".to_string(),
        "500" => "INTERNAL SERVER ERROR".to_string(),
        _ => "INTERNAL SERVER ERROR".to_string(),
    };

    debug!("Status Message: {}", status_msg);

    let contents = fs::read_to_string(filename).unwrap();

    let status_line = format!("HTTP/1.1 {} {}\r\n\r\n", code, status_msg);
    format!("{}{}", status_line, contents)
}

// TODO: mimetype for stataic files, dupport dynamic content
pub fn handle_request(filename: String, req: &httparse::Request) -> String {
    let status_line = "HTTP/1.1 200 OK\r\n\r\n";
    let contents = fs::read_to_string(filename).unwrap();

    format!("{}{}", status_line, contents)
}
