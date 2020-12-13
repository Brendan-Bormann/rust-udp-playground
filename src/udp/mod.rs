use std::net::UdpSocket;

pub mod packet;
use packet::*;

pub struct UDPConnection {
    host_address: String,
    socket: UdpSocket
}

impl UDPConnection {
    pub fn new_server(host_addr: &str) -> UDPConnection {
        println!("Creating UDP Server...");

        let socket = match UdpSocket::bind(host_addr) {
            Ok(socket) => {
                println!("- Server started at: [{}]", host_addr);
                socket
            },
            Err(_) => panic!("- Failed to bind server to port.")
        };
        socket.set_nonblocking(true).expect("- Failed to set server to non-blocking.");

        println!("");

        UDPConnection {
            host_address: host_addr.to_string(),
            socket
        }
    }

    pub fn new_client(host_addr: &str, server_sddr: &str) -> UDPConnection {
        println!("Creating UDP Client...");

        let socket = match UdpSocket::bind(host_addr) {
            Ok(socket) => {
                println!("- Client started at: [{}]", host_addr);
                socket
            },
            Err(_) => panic!("- Failed to bind client to port.")
        };
        socket.set_nonblocking(true).expect("- Failed to set client to non-blocking.");

        match socket.connect(server_sddr) {
            Ok(socket) => {
                println!("- Connected to server at [{}]", server_sddr);
                socket
            },
            Err(_) => panic!("- Failed to connect to server.")
        };

        println!("");

        UDPConnection {
            host_address: host_addr.to_string(),
            socket
        }
    }

    pub fn listen(&self, message_handler: fn(packet: Packet)) {

        loop {
            let mut buf = [0; 512];
    
            match &self.socket.recv_from(&mut buf) {
                
                Ok((size, _addr)) => {
                    let data = buf.to_vec();
                    let packet = Packet::deserialize_packet(&data);

                    message_handler(packet);

                    println!("Packet size: {}", size);
                },
    
                Err(error) => match error.kind() {
                    std::io::ErrorKind::WouldBlock => (),
                    x => println!("{:?}", x)
                }
            }
    
        };
    }

    pub fn send_message(&self, target_addr: &str, message: String) -> Result<usize, String> {
        
        let new_packet = Packet {
            sender: self.host_address.to_string(),
            packet_type: "message".to_string(),
            packet_data: message
        };

        let buf = Packet::serialize_packet(&new_packet);

        match self.socket.send_to(&buf, target_addr) {
            Ok(size) => Ok(size),
            Err(error) => Err(error.to_string())
        }
    }

    pub fn clone(&self) -> UDPConnection {

        let socket_clone = match self.socket.try_clone() {
            Ok(socket) => socket,
            Err(_) => panic!("└ Failed to clone socket for client.")
        };

        UDPConnection {
            host_address: self.host_address.to_string(),
            socket: socket_clone
        }
    }
}

