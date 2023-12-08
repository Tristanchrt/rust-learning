use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}


struct Node {
    data: String,
    next: Option<Box<Node>>
}

struct Linkedlist {
    head: Option<Box<Node>>,
}

fn main() {
    let _ = read_username_from_file();
}