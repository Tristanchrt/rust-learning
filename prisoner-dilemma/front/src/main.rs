use std::io::{Read, Write};
use std::net::TcpStream;
use settings::{Settings, Log};


fn start_client() {
    let mut tcp_stream = TcpStream::connect("127.0.0.1:9001").unwrap();
    tcp_stream.write_all(b"Hello server").unwrap();
    tcp_stream.flush().unwrap();
    let mut buffer = [0; 1024];
    let size = tcp_stream.read(&mut buffer).unwrap();
    let message = String::from_utf8_lossy(&buffer[..size]);
    println!("Server says: {}", message);
}


fn main() {
    // start_client();

    let a = format!("{} {} toto", 5, 01);
    Log::show("INFO", a);

    let settings = Settings::load("../settings/settings.json");
    println!("toto {:?}", settings);
}
