use std::fs;
use super::network::send_tcp_message;
use super::network::receive_response;

pub fn send_file(ip_address: &str, port: u16, file_path: &str) -> Vec<u8> {

    let mut response = Vec::new();

    let file_content = match fs::read(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            return response;
        }
    };

    response = send_tcp_message(ip_address, port, &file_content);
    
    response
}
