use std::io::{Read, Write};
use std::net::TcpListener;


fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:9001").unwrap();
    for stream in listener.incoming() {
        let mut tcp_stream = stream.unwrap();
        let mut buffer = [0; 1024];
        let _size = tcp_stream.read(&mut buffer).unwrap();
        tcp_stream.write_all(b"Hello, client").unwrap();
        tcp_stream.flush().unwrap();
    }
}


fn main() {
    start_server();
}
