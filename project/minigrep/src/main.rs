use std::env;

use minigrep;

fn main() {
    let args: Vec<String> = env::args().collect();

    let configuration = match minigrep::Config::new(&args) {
        Ok(config) => {
            println!("{}", config.to_string());
            Some(config)
        }
        Err(error) => {
            eprintln!("Error: {:?}", error);
            None
        }
    };

    if let Some(config) = configuration {
        let _ = minigrep::run(config);
    } else {
        eprintln!("Configuration is None. Cannot run minigrep.");
    }
}
