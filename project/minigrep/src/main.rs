use std::env;

use minigrep;

fn main() {
    let configuration = match minigrep::Config::build(env::args()) {
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
