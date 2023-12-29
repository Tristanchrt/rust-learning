slint::include_modules!();
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

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();

    ui.on_event_game(move |data| {
        Log::show("INFO", data.to_string());
    });

    ui.set_menu_visible(false);
    ui.set_search_visible(false);
    ui.set_create_visible(true);
    ui.set_wait_visible(false);
  
    ui.set_number_bet(10);
    ui.set_number_round(5);

    let ui_clone = ui.clone_strong();
    ui.on_create_game(move || {
        let round = ui_clone.get_number_round();
        let bet = ui_clone.get_number_bet();
        println!("AAAA {} {}", round, bet);
    });

    ui.run()
}
