
slint::include_modules!();
use std::net::TcpStream;
use std::io::{Read, Write};
use settings::{Settings, Log};
use std::thread;

pub struct Interface {
    pub ui: AppWindow,

}

pub struct Client {
    pub tcp: TcpStream,
}

pub struct Controller {
    pub settings: Settings
}

impl Client {
    pub fn new(host: &str, port: &str) -> Self {
        let tcp = TcpStream::connect(String::from(host.to_string() + ":" + port)).unwrap();
        Self { tcp }
    }

    fn init(mut self) {
        self.tcp.write_all(b"Hello server").unwrap();
        self.tcp.flush().unwrap();
        let mut buffer = [0; 1024];
        let size = self.tcp.read(&mut buffer).unwrap();
        let message = String::from_utf8_lossy(&buffer[..size]);
        println!("Server says: {}", message);
    }

    fn send_message(&self) {

    }
}

impl Controller {
    pub fn run(&self) {
        let ui = Interface::new();
        ui.init();

        let client = Client::new(&self.settings.host, &self.settings.port);

        client.init();

        ui.run();
    }
}

impl Interface {

    pub fn new() -> Self {
        let ui = AppWindow::new().unwrap();
        Self { ui }
    }

    fn init(&self) {
        self.set_default_input();
        self.reset_interface();
        self.attach_event_handlers();
    }

    fn attach_event_handlers(&self) {
        self.ui.on_event_game(move |data| {
            Log::show("INFO", data.to_string());
        });
    
        let ui_clone = self.ui.clone_strong();
        self.ui.on_create_game(move || {
            let round = ui_clone.get_number_round();
            let bet = ui_clone.get_number_bet();
            println!("AAAA {} {}", round, bet);
        });
    }

    fn set_default_input(&self) {
        self.ui.set_number_bet(10);
        self.ui.set_number_round(5);
    }

    fn reset_interface(&self){
        self.ui.set_menu_visible(true);
        self.ui.set_game_visible(false);
        self.ui.set_search_visible(false);
        self.ui.set_create_visible(false);
        self.ui.set_wait_visible(false);
    }

    fn run(&self) {
        let _ = self.ui.run();
    }

}
