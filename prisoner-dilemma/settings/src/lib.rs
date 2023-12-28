use config::Config;
use std::collections::HashMap;

#[derive(Debug)]

pub struct Settings {
    pub host: String, 
    pub port: String,
    pub client_max: String,
    pub buffer_size1: String,
    pub buffer_size2: String,
    pub party_name: String
}

pub struct Log;

impl Log {
    pub fn show(key: &str, value: String) {
        match key {
            "WARN" => println!("** [WARN] {} **", value),
            "DEBUG" => println!("** [DEBUG] {} **", value),
            "INFO" => println!("** [INFO] {} **", value),
            "ERROR" => println!("** [ERROR] {} **", value),
            _ => println!("{}", value),
        }
    }
}

impl Settings {
    
    pub fn load(file_name: &str) -> Self {
        let settings = Config::builder()
            .add_source(config::File::with_name(file_name))
            .build()
            .unwrap();

        let settings_map = settings.try_deserialize::<HashMap<String, String>>().unwrap();

        Self {
            host: Self::get_configuration_value(&settings_map, "host"),
            port: Self::get_configuration_value(&settings_map, "port"),
            client_max: Self::get_configuration_value(&settings_map, "client_max"),
            buffer_size1: Self::get_configuration_value(&settings_map, "buffer_size1"),
            buffer_size2: Self::get_configuration_value(&settings_map, "buffer_size2"),
            party_name: Self::get_configuration_value(&settings_map, "party_name"),
        }
    }
    
    pub fn get_configuration_value(dict: &HashMap<String, String>, key: &str) -> String {
        match dict.get(key) {
            Some(value) => value.to_string(),
            None => {
                Log::show("ERROR", format!("Key {} not found in the configuration file.", key));
                String::new()
            }
        }
    }
  
}



