use std::{env::args, println};
use std::thread;

mod packets;
mod udp;

const SERVER_ADDR: &str = "127.0.0.1:8080";
const LOCAL_ADDR: &str = "127.0.0.2:8080";

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() > 1 && args[1] == "server" {
        start_server();
    } else {
        start_client();
    }
}

fn start_server() {
    let server = udp::UDPConnection::new_server(SERVER_ADDR);
    server.listen(server_message_handler);
}

fn server_message_handler(message: String, addr: String) {
    println!("Message: [{}]\n- {}", addr, message);
}

fn start_client() {
    let client = udp::UDPConnection::new_client(LOCAL_ADDR, SERVER_ADDR);

    let client_clone = client.clone();

    thread::spawn(move || {
        client_clone.listen(server_message_handler);
    });

    match client.send_message(SERVER_ADDR, "Hello".to_string()) {
        Ok(_) => println!("Sent message."),
        Err(_) => println!("Failed to send message.")
    };
}