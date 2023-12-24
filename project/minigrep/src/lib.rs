use std::error::Error;
use std::fs;

/// Structure to hold configuration data for search operation
pub struct Config {
    /// Query string used for searching
    pub query: String,
    /// File path where search operation will be performed
    pub file_path: String,
}

/// Enum representing configuration-related errors
#[derive(Debug)]
pub enum ConfigError {
    /// Error indicating not enough arguments were provided
    NotEnoughArguments,
    /// Error indicating the file was not found
    FileNotFound,
}

impl Config {
    /// Builder function to create a Config instance from command-line arguments
    ///
    /// # Arguments
    ///
    /// * `args` - Iterator of strings representing command-line arguments
    ///
    /// # Returns
    ///
    /// A `Result` containing either a `Config` instance or an error message
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // Skip the program name

        // Extract query string from arguments
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        // Extract file path from arguments
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        // Create and return a Config instance
        Ok(Config { query, file_path })
    }

    /// Returns a formatted string representation of the Config data
    pub fn to_string(&self) -> String {
        format!("Query {} File Path {}", self.query, self.file_path)
    }
}

/// Executes the search process based on the given Config
///
/// # Arguments
///
/// * `config` - A `Config` instance containing search parameters
///
/// # Returns
///
/// A `Result` indicating success or an error message
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read contents of the file
    let contents = fs::read_to_string(config.file_path)?;

    // Search for lines containing the query string
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

/// Searches for lines containing the query in the given contents
///
/// # Arguments
///
/// * `query` - The query string to search for
/// * `contents` - The contents to search within
///
/// # Returns
///
/// A vector containing lines that match the query
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines() // Iterate through lines of the content
        .filter(|line| line.contains(query)) // Filter lines containing the query
        .collect() // Collect the lines into a vector
}
