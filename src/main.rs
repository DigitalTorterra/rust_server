extern crate threadpool;
extern crate log;
extern crate serde_json;

use std::net::TcpListener;
use std::env;
use std::process;
use std::sync::Arc;
use threadpool::ThreadPool;
use log::{info, warn};
use serde_json::Value;

mod lib;
use lib::{ handle_connection, parse_site_structure };
mod config;
use config::Config;

fn main() {
    env_logger::init();

    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);    
    });

    if config.help {
        config.print_help();
        process::exit(0);
    }

    let ip = "127.0.0.1:".to_string() + &config.port;
    info!("Listening on {}", ip);


    let listener = TcpListener::bind(ip).unwrap(); 
    let pool = ThreadPool::new(config.workers);

    let site_structure = Arc::new(parse_site_structure(&config.root).unwrap_or_else(|err| {
        eprintln!("Invalid structure.json file!");
        process::exit(1);
    }));
    let root = Arc::new(config.root);

    for stream in listener.incoming() {
        info!("Receiving incoming connection");

        let stream = stream.unwrap();
        let site_structure = Arc::clone(&site_structure);
        let root = Arc::clone(&root);

        
        pool.execute(move || {
            handle_connection(stream, root, site_structure);
        });
        
    }

    println!("Shutting down.");
}


