use std::net::UdpSocket;

pub struct UDPConnection {
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
            Err(msg) => panic!("- Failed to bind server to port.")
        };
        socket.set_nonblocking(true).expect("- Failed to set server to non-blocking.");

        println!("");

        UDPConnection { socket }
    }

    pub fn new_client(host_addr: &str, server_sddr: &str) -> UDPConnection {
        println!("Creating UDP Client...");

        let socket = match UdpSocket::bind(host_addr) {
            Ok(socket) => {
                println!("- Client started at: [{}]", host_addr);
                socket
            },
            Err(msg) => panic!("- Failed to bind client to port.")
        };
        socket.set_nonblocking(true).expect("- Failed to set client to non-blocking.");

        match socket.connect(server_sddr) {
            Ok(socket) => {
                println!("- Connected to server at [{}]", server_sddr);
                socket
            },
            Err(msg) => panic!("- Failed to connect to server.")
        };

        println!("");

        UDPConnection { socket }
    }

    pub fn listen(&self, message_handler: fn(message: String, origin_addr: String)) {

        loop {
            let mut buf = [0; 512];
    
            match &self.socket.recv_from(&mut buf) {
                
                Ok((size, addr)) => {
                    let data = buf.to_vec();
    
                    match String::from_utf8(data) {
                        Ok(m) => message_handler(m, addr.to_string()),
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

    pub fn send_message(&self, target_addr: &str, message: String) -> Result<usize, String> {
        let buffer = message.as_bytes();

        match self.socket.send_to(buffer, target_addr) {
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
            socket: socket_clone
        }
    }
}