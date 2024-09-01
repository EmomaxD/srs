use std::env;
use std::fs::{File, self};
use std::io::{Read, Write};
use std::net::{TcpStream, UdpSocket};
use std::process;

fn parse_address(address_str: &str) -> Result<(String, u16), String> {
    let parts: Vec<&str> = address_str.split(':').collect();
    if parts.len() != 2 {
        return Err("Invalid address format. Expected IP:PORT".to_string());
    }

    let ip_str = parts[0];
    let port_str = parts[1];

    let ip_parts: Vec<&str> = ip_str.split('.').collect();
    if ip_parts.len() != 4 {
        return Err("Invalid IP address format".to_string());
    }

    for part in ip_parts {
        let _octet: u8 = match part.parse() {
            Ok(value) if value <= 255 => value,
            _ => return Err("Invalid IP address octet".to_string()),
        };
    }

    let port: u16 = match port_str.parse() {
        Ok(value) if value <= 65535 => value,
        _ => return Err("Invalid port number".to_string()),
    };

    Ok((ip_str.to_string(), port))
}

fn send_udp_message(ip_address: &str, port: u16, message: &str) {
    let target = format!("{}:{}", ip_address, port);
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");

    match socket.send_to(message.as_bytes(), &target) {
        Ok(size) => println!("Sent {} bytes over UDP", size),
        Err(err) => eprintln!("Failed to send UDP data: {}", err),
    }
}

fn send_tcp_message(ip_address: &str, port: u16, message: &[u8]) -> Vec<u8> {
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

fn send_file(ip_address: &str, port: u16, file_path: &str) {
    let file_content = match fs::read(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            return;
        }
    };

    let response = send_tcp_message(ip_address, port, &file_content);
    receive_response(&response, None); // Print the response if any
}

fn receive_response(response: &[u8], save_path: Option<&str>) {
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

fn send_http_request(ip_address: &str, port: u16) -> Vec<u8> {
    let target = format!("{}:{}", ip_address, port);
    let request = format!(
        "GET / HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        ip_address
    );
    let mut response = Vec::new();

    match TcpStream::connect(&target) {
        Ok(mut stream) => {
            if let Err(e) = stream.write_all(request.as_bytes()) {
                eprintln!("Failed to send HTTP request: {}", e);
                return response; // Return empty response on failure
            }

            if let Err(e) = stream.read_to_end(&mut response) {
                eprintln!("Failed to read HTTP response: {}", e);
            }
        }
        Err(err) => eprintln!("Failed to connect to HTTP server: {}", err),
    }

    response
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <IP:PORT> <tcp|udp> [message|file_path] [--save <path>]", args[0]);
        process::exit(1);
    }

    let address_str = &args[1];
    let mode = &args[2];
    let file_path = if args.len() > 3 && args[3].ends_with(".txt") { &args[3] } else { "" };
    let message = if args.len() > 3 && !args[3].ends_with(".txt") { &args[3] } else { "" };
    let save_path = args.iter().position(|arg| arg == "--save").and_then(|i| args.get(i + 1).map(|s| s.as_str()));

    let (ip_address, port) = match parse_address(address_str) {
        Ok((ip, port)) => (ip, port),
        Err(e) => {
            eprintln!("Error parsing address: {}", e);
            process::exit(1);
        }
    };

    let response = match mode.as_str() {
        "udp" => {
            if !file_path.is_empty() {
                eprintln!("UDP mode does not support file transmission.");
                process::exit(1);
            }
            send_udp_message(&ip_address, port, message);
            Vec::new() // No response for UDP
        }
        "tcp" => {
            if !file_path.is_empty() {
                send_file(&ip_address, port, file_path);
                Vec::new() // File sending does not expect a response
            } else {
                if message == "http" {
                    send_http_request(&ip_address, port)
                } else {
                    send_tcp_message(&ip_address, port, message.as_bytes())
                }
            }
        }
        _ => {
            eprintln!("Invalid mode. Use 'tcp' or 'udp'");
            process::exit(1);
        }
    };

    if !response.is_empty() || save_path.is_some() {
        receive_response(&response, save_path);
    }
}
