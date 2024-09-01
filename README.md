# SRS (Sockets Rust)

**SRS** is a command-line tool written in Rust that enables multi-threaded concurrent network communication via TCP and UDP, including file transfer and HTTP/HTTPS requests.

## Features

- **TCP Communication**: Send and receive messages over TCP.
- **UDP Communication**: Send messages over UDP.
- **File Transfer**: Send files over TCP.
- **HTTP/HTTPS Requests**: Send HTTP or HTTPS GET requests and receive responses.
- **Concurrent Multi-Threaded Processing**: Handle multiple network operations concurrently using thread pool.

## Running

```bash
   cargo run <URL|IP:PORT> <tcp|udp|http|https> [--message <message>] [--file <file_path>] [--count <request_count>] [--save <path>]
```

## Example usages

```bash
   cargo run www.google.com https --save google.html
```

```bash
   cargo run 127.0.0.1:8080 udp --message "Hello from client!"
```

```bash
   cargo run 127.0.0.1:8080 tcp --file file.txt --save response
```

```bash
   cargo run 127.0.0.1:8080 tcp --count 15000 --message "Lots of concurrent requests from client using thread pool!"
```