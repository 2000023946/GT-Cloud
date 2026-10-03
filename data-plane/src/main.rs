mod api;
mod applications;
mod bootstrap;
mod domain;
mod infrastructure;
mod ports;

use std::net::TcpListener;

fn main() {
    let api = bootstrap::bootstrap::start();

    println!("Data plane started");

    let _ = api.send_heartbeat();

    let listener = TcpListener::bind("0.0.0.0:8081")
        .expect("Failed to bind to port 8081");

    println!("Data plane listening on port 8081");

    for stream in listener.incoming() {
        match stream {
            Ok(_) => {
                println!("Received connection");
            }
            Err(error) => {
                println!("Connection failed: {}", error);
            }
        }
    }
}