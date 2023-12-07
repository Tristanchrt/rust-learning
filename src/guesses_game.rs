use rand::Rng;
use std::cmp::Ordering;
use std::io;

mod messages;

use messages::{print_too_small, print_you_win, print_too_big};

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop { 

        println!("Please input your guess.");
        
        let mut guess = String::new();

        let spaces = "   ";
        let spaces = &spaces.len();
        
        println!("Test: {spaces}");

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("Must be string\n");
                continue;
            }
        };


        println!("You guessed: {guess} {THREE_HOURS_IN_SECONDS}");

        match guess.cmp(&secret_number) {
            Ordering::Less => print_too_small(),
            Ordering::Greater => print_too_big(),
            Ordering::Equal => {
                print_you_win();
                break
            },

        }
    }

}