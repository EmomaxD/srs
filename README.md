# SRS (Sockets Rust)

**SRS** is a command-line tool written in Rust that enables network communication via TCP and UDP, including file transfer and HTTP requests.

## Features

- **TCP Communication**: Send and receive messages over TCP.
- **UDP Communication**: Send messages over UDP.
- **File Transfer**: Send files over TCP.
- **HTTP Requests**: Send HTTP GET requests and receive responses.

## Running

```bash
   cargo run <IP:PORT> <tcp|udp> [message|file_path] [--save <path>]
```