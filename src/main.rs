use std::env;
use std::process;

mod network;
mod file_utils;
mod parser;

use network::{send_udp_message, send_tcp_message, send_http_request, receive_response};
use parser::parse_address;
use file_utils::send_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <URL|IP:PORT> <tcp|udp|http|https> [message|file_path] [--save <path>]", args[0]);
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

    match mode.as_str() {
        "udp" => {
            if !file_path.is_empty() {
                eprintln!("UDP mode does not support file transmission.");
                process::exit(1);
            }
            send_udp_message(&ip_address, port, message);
        }
        "tcp" => {
            if !file_path.is_empty() {
                send_file(&ip_address, port, file_path);
            } else {
                if message == "http" || message == "https" {
                    eprintln!("HTTP(S) mode does not expect a message. Use a URL instead.");
                    process::exit(1);
                } else {
                    let response = send_tcp_message(&ip_address, port, message.as_bytes());
                    receive_response(&response, save_path);
                }
            }
        }
        "http" | "https" => {
            let url = if mode == "http" {
                format!("http://{}", address_str)
            } else {
                format!("https://{}", address_str)
            };
            let response = send_http_request(&url);
            receive_response(&response, save_path);
        }
        _ => {
            eprintln!("Invalid mode. Use 'tcp', 'udp', 'http', or 'https'");
            process::exit(1);
        }
    }
}
