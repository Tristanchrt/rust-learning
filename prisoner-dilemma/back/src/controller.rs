use settings::{Party, Settings, Status};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub struct Controller {
    pub listener: TcpListener,
}

impl Controller {
    pub fn new(settings: &Settings) -> Self {
        let listener =
            TcpListener::bind(String::from(format!("{}:{}", settings.host, settings.port)))
                .unwrap();
        Self { listener }
    }

    pub fn run(&self) {
        for stream in self.listener.incoming() {
            self.process_message(stream.unwrap());
        }
    }

    pub fn process_message(&self, mut tcp_stream: TcpStream) {
        let mut buffer = [0; 1024];
        let _size = tcp_stream.read(&mut buffer).unwrap();

        if _size == 0 {
            panic!("Error size 0");
        }

        tcp_stream.write_all(b"Hello, client").unwrap();
        tcp_stream.flush().unwrap();

        // let party = self.deserialization(tcp_stream);
        // self.handle_party(party);
    }

    pub fn handle_party(&self, party: Party) {
        match party.status {
            Status::Created => println!("1"),
            Status::Started => println!("2"),
            Status::WaitingPlayer => println!("3"),
            Status::Finished => println!("4"),
            _ => println!("Something went wrong with the Party status"),
        }
    }

    pub fn deserialization(&self, tcp_stream: TcpStream) -> Party {
        Party {
            id: todo!(),
            game_id: todo!(),
            total_round: todo!(),
            round: todo!(),
            status: todo!(),
            bet: todo!(),
            players: todo!(),
            winner: todo!(),
            looser: todo!(),
        }
    }
}
