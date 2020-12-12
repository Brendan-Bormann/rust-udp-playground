use std::{net::UdpSocket, println};
use std::env::args;

mod packets;

const SERVER_ADDR: &str = "127.0.0.1:8080";
const LOCAL_ADDR: &str = "127.0.0.1:8080";

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() > 1 && args[1] == "server" {
        start_server();
    } else {
        start_client();
    }
}

fn start_server() {
    println!("Starting UDP Server. Target address: [{}]", SERVER_ADDR);

    let socket = UdpSocket::bind(SERVER_ADDR).expect("Failed to bind server to port.");
    socket.set_nonblocking(true).expect("Failed to set server to non-blocking.");

    let mut message_count = 0;

    loop {
        let mut buf = [0; 512];

        match socket.recv_from(&mut buf) {
            
            Ok((size, addr)) => {
                let data = buf.to_vec();
                println!("From {} - Size: {}", addr, size);

                match String::from_utf8(data) {
                    Ok(m) => {
                        println!("Message: {}", m);

                        message_count = message_count + 1;

                        println!("Total messages: {}", message_count);

                        let out_buf = m.as_bytes();

                        socket.send_to(out_buf, addr).expect("Failed to send response.");
                    },
                    Err(e) => println!("Error: {}", e)
                }

            },

            Err(error) => match error.kind() {
                std::io::ErrorKind::WouldBlock => (),
                x => println!("{:?}", x)
            }
        }

    };
}

fn start_client() {
    println!("Starting UDP Client.");

    let socket = UdpSocket::bind(LOCAL_ADDR).expect("Failed to bind server to port.");
    socket.set_nonblocking(true).expect("Failed to set server to non-blocking.");

    socket.connect(SERVER_ADDR).expect("Failed to connect to server.");

    let socket_clone = socket.try_clone().expect("Failed to clone socket.");

    std::thread::spawn(move || {

        loop {
            let mut buf = [0; 512];
    
            match socket_clone.recv_from(&mut buf) {
                
                Ok((size, addr)) => {
                    let data = buf.to_vec();
                    println!("From {}:", addr);
    
                    match String::from_utf8(data) {
                        Ok(m) => {
                            println!("Message: {}", m);
                        },
                        Err(e) => println!("Error: {}", e)
                    }
    
                },
    
                Err(error) => match error.kind() {
                    std::io::ErrorKind::WouldBlock => (),
                    x => println!("{:?}", x)
                }
            }
    
        };
    });

    loop {
        let message = "Hello, world!".to_string();

        let buf = message.as_bytes();

        socket.send(&buf).expect("Failed to send message.");
    }
}