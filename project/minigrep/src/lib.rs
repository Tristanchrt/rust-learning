use std::fs;
use std::error::Error;


pub struct Config {
    query: String,
    file_path: String,
}

#[derive(Debug)]
pub enum ConfigError {
    NotEnoughArguments,
    FileNotFound
}

impl Config { 
    pub fn new(args: &[String]) -> Result<Config, ConfigError> {

        if args.len() < 3 {
            return Err(ConfigError::NotEnoughArguments);
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        if !std::path::Path::new(&file_path).exists() {
            return Err(ConfigError::FileNotFound);
        }
        
        Ok(Config {query, file_path})
    }
    pub fn to_string(&self) -> String {
        return format!("Query {} File Path {}", self.query, self.file_path);
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    for line in search(&config.query, &contents) {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}
