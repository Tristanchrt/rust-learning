use rand::Rng;
use settings::{Game, Log, Party, Protocol, Settings, Status};
use std::io::{Bytes, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::result;

pub struct Controller {
    pub listener: TcpListener,
    pub game: Game,
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts((p as *const T) as *const u8, ::core::mem::size_of::<T>())
}

type BufferSize = [u8; 1024];

impl Controller {
    pub fn new(settings: &Settings) -> Self {
        let listener =
            TcpListener::bind(String::from(format!("{}:{}", settings.host, settings.port)))
                .unwrap();
        let game = Game::default();
        Self { listener, game }
    }

    pub fn run(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(result) => self.process_message(result),
                Err(_) => Log::show(
                    "ERROR",
                    format!("Someting went wrong for reading the stream"),
                ),
            }
        }
    }

    pub fn process_message(&self, mut tcp_stream: TcpStream) {
        let mut buffer: BufferSize = [0; 1024];
        let _size = tcp_stream.read(&mut buffer).unwrap();

        if _size == 0 {
            panic!("Error size 0");
        }
        let protocol: Protocol = Protocol::from_bytes(&buffer[.._size]);

        self.handle_party(&protocol, &tcp_stream);
    }

    pub fn handle_party(&self, protocol: &Protocol, tcp_stream: &TcpStream) {
        match protocol.party_status {
            Status::Init => self.init_player(&tcp_stream),
            Status::Created => println!("1"),
            Status::Started => println!("2"),
            Status::WaitingPlayer => println!("3"),
            Status::Finished => println!("4"),
            _ => println!("Something went wrong with the Party status"),
        }
    }

    pub fn init_player(&self, mut tcp_stream: &TcpStream) {
        let mut protocol: Protocol = Protocol::default();
        let mut rng = rand::thread_rng();
        protocol.player.id = rng.gen::<u32>();

        Log::show("INFO", format!("New user #{}", protocol.player.id));

        let bytes = protocol.to_bytes();

        tcp_stream.write_all(&bytes).unwrap();
        tcp_stream.flush().unwrap();
    }
}
