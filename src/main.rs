mod config;
mod header;
mod attributes;
mod handlers;

use std::io::{ Read, Write };
use std::net::{ TcpListener, UdpSocket, SocketAddr };
use std::thread;

fn main() {
    let config = config::get_config("config.toml");

    // Start UDP listener
    let udp_socket = match UdpSocket::bind(format!("0.0.0.0:{}", config.port)) {
        Ok(udp_socket) => udp_socket,
        Err(_) => {
            println!("unable to establish udp socket at {}", config.port);
            return;
        },
    };
    // Spawn thread for UDP
    thread::spawn(move || {
        loop {
            let mut buf = [0; 548];
            match udp_socket.recv_from(&mut buf) {
                Ok((_amt, src)) => match process_message(&buf, &src) {
                    Some(res) => match udp_socket.send_to(&res, &src) {
                        _ => continue,
                    },
                    None => continue,
                },
                Err(_) => continue,
            };
        }
    });

    // Start TCP listener
    let tcp_listener = match TcpListener::bind(format!("0.0.0.0:{}", config.port)) {
        Ok(tcp_listener) => tcp_listener,
        Err(_) => {
            println!("unable to establish tcp listener at {}", config.port);
            return;
        },
    };
    // Handle TCP on main thread
    for stream in tcp_listener.incoming() {
        let mut stream = match stream {
            Ok(stream) => stream,
            Err(_) => continue,
        };
        let mut buf = [0; 548];
        let peer_addr = match stream.peer_addr() {
            Ok(peer_addr) => peer_addr,
            Err(_) => continue,
        };
        match stream.read(&mut buf) {
            Ok(_) => match process_message(&buf, &peer_addr) {
                Some(res) => match stream.write(&res) {
                    _ => continue,
                },
                None => continue,
            },
            Err(_) => continue,
        };
    }
}

fn process_message(message: &[u8; 548], src: &SocketAddr) -> Option<Vec<u8>> {
    let header = match header::verify_header(&message[0..20]) {
        Ok(header) => header,
        Err(_) => return None,
    };
    let body = &message[20..(20 + header.length as usize)];
    let attributes = attributes::get_attributes(body, &header);
    
    handlers::process_message(&header, &attributes, src)
}
