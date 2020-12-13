use std::env::args;
use std::io::{self, BufRead};
use std::thread;

mod udp;
use udp::packet;

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

fn server_message_handler(packet: packet::Packet) {
    println!("Message: [{}]\n- {}", packet.sender, packet.packet_data);
}

fn start_client() {
    let client = udp::UDPConnection::new_client(LOCAL_ADDR, SERVER_ADDR);

    let client_clone = client.clone();

    thread::spawn(move || {
        client_clone.listen(server_message_handler);
    });

    loop {
        
        let message = read_stdin();

        match client.send_message(SERVER_ADDR, message) {
            Ok(_) => println!("Sent message."),
            Err(error) => println!("Error: {}", error)
        };

    }
}

fn read_stdin() -> String {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();

    line
}