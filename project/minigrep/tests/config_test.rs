
use minigrep::{Config, ConfigError, search};


#[cfg(test)]
mod tests_config {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn file_not_found() {
        let args = vec!["minigrep".to_string(), "query".to_string(), "non_existing_file.txt".to_string()];
        match Config::new(&args) {
            Err(ConfigError::FileNotFound) => assert!(true), // Expected error variant
            _ => assert!(false), // Any other result is considered a failure
        }
    }
}