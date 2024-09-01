use std::env;
use std::process;
use num_cpus;
use threadpool::ThreadPool;

mod network;
mod file_utils;
mod parser;

use network::{send_udp_message, send_tcp_message, send_http_request, receive_response};
use parser::parse_address;
use file_utils::send_file;

fn handle_request(address_str: &str, mode: &str, message: &str, file_path: &str, save_path: Option<&str>) {
    let (ip_address, port) = match parse_address(address_str) {
        Ok((ip, port)) => (ip, port),
        Err(e) => {
            eprintln!("Error parsing address: {}", e);
            process::exit(1);
        }
    };

    match mode {
        "udp" => {
            if !file_path.is_empty() {
                eprintln!("UDP mode does not support file transmission.");
                process::exit(1);
            }
            send_udp_message(&ip_address, port, message);
        }
        "tcp" => {
            if !file_path.is_empty() {
                let response = send_file(&ip_address, port, file_path);
                receive_response(&response, save_path);
            }
            if !message.is_empty() {
                let response = send_tcp_message(&ip_address, port, message.as_bytes());
                receive_response(&response, save_path);
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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <URL|IP:PORT> <tcp|udp|http|https> [--message <message>] [--file <file_path>] [--count <request_count>] [--save <path>]", args[0]);
        process::exit(1);
    }

    let address_str = args[1].clone();
    let mode = args[2].clone();

    let mut message = String::new();
    let mut file_path = String::new();
    let mut request_count = 1;
    let mut save_path: Option<String> = None;

    let mut i = 3;
    while i < args.len() {
        match args[i].as_str() {
            "--message" => {
                if i + 1 < args.len() {
                    message = args[i + 1].clone();
                    i += 1;
                } else {
                    eprintln!("Expected a message after '--message'");
                    process::exit(1);
                }
            }
            "--file" => {
                if i + 1 < args.len() {
                    file_path = args[i + 1].clone();
                    i += 1;
                } else {
                    eprintln!("Expected a file path after '--file'");
                    process::exit(1);
                }
            }
            "--count" => {
                if i + 1 < args.len() {
                    request_count = args[i + 1].parse().unwrap_or_else(|_| {
                        eprintln!("Invalid request count value");
                        process::exit(1);
                    });
                    i += 1;
                } else {
                    eprintln!("Expected a request count after '--count'");
                    process::exit(1);
                }
            }
            "--save" => {
                if i + 1 < args.len() {
                    save_path = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Expected a save path after '--save'");
                    process::exit(1);
                }
            }
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
                process::exit(1);
            }
        }
        i += 1;
    }

    let num_cores = num_cpus::get();
    println!("Using {} cores, {} threads", num_cores, num_cores * 2);
    
    let pool = ThreadPool::new(num_cores * 2);

    for _ in 0..request_count {
        let address_str = address_str.clone();
        let mode = mode.clone();
        let message = message.clone();
        let file_path = file_path.clone();
        let save_path = save_path.clone();

        pool.execute(move || {
            handle_request(&address_str, &mode, &message, &file_path, save_path.as_deref());
        });
    }

    pool.join();  // Wait for all threads to complete
}
