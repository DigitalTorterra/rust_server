extern crate threadpool;

use std::net::TcpListener;
use std::env;
use std::process;
use rust_server::{ handle_connection, Config };
//use threadpool::ThreadPool;

fn main() {
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
    let listener = TcpListener::bind(ip).unwrap();
    
    //let n_workers = 4;
    //let pool = ThreadPool::new(n_workers);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        /*
        pool.execute(|| {
            handle_connection(stream);
        });
        */
        handle_connection(stream)
    }

    println!("Shutting down.");
}


