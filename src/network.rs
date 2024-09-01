use reqwest::blocking::Client;
use reqwest::Url;
use std::io::{Read, Write};
use std::net::{TcpStream, UdpSocket};
use std::fs::File;

pub fn send_udp_message(ip_address: &str, port: u16, message: &str) {
    let target = format!("{}:{}", ip_address, port);
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");

    match socket.send_to(message.as_bytes(), &target) {
        Ok(size) => println!("Sent {} bytes over UDP", size),
        Err(err) => eprintln!("Failed to send UDP data: {}", err),
    }
}

pub fn send_tcp_message(ip_address: &str, port: u16, message: &[u8]) -> Vec<u8> {
    let target = format!("{}:{}", ip_address, port);
    let mut response = Vec::new();

    match TcpStream::connect(&target) {
        Ok(mut stream) => {
            if let Err(e) = stream.write_all(message) {
                eprintln!("Failed to send TCP data: {}", e);
                return response; // Return empty response on failure
            }

            if let Err(e) = stream.read_to_end(&mut response) {
                eprintln!("Failed to read response: {}", e);
            }
        }
        Err(err) => eprintln!("Failed to connect to TCP server: {}", err),
    }

    response
}

pub fn send_http_request(url_str: &str) -> Vec<u8> {
    let client = Client::new();
    let mut response = Vec::new();

    match client.get(url_str).send() {
        Ok(mut resp) => {
            if let Err(e) = resp.copy_to(&mut response) {
                eprintln!("Failed to read HTTP response: {}", e);
            }
        }
        Err(err) => eprintln!("Failed to send HTTP request: {}", err),
    }

    response
}

pub fn receive_response(response: &[u8], save_path: Option<&str>) {
    // Print the received response
    println!("Received response:\n{}", String::from_utf8_lossy(response));

    // Save the response to a file if a save path is provided
    if let Some(path) = save_path {
        match File::create(path) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(response) {
                    eprintln!("Failed to save response: {}", e);
                } else {
                    println!("Response saved to {}", path);
                }
            }
            Err(e) => eprintln!("Failed to create file: {}", e),
        }
    }
}
