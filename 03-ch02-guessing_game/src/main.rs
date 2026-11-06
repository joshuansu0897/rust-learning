// Standard
use std::cmp::Ordering;
use std::io;

// Dependencies
use rand::prelude::*;

fn main() {
    // Tittle
    println!("##############################");
    println!("#      Guess the number!     #");
    println!("##############################");
    println!();

    // Generate the random number
    let secret_number = rand::rng().random_range(1..=100);

    // Request user input
    loop {
        println!("Please input your guess: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Parse the input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!();
                println!("TYPE A NUMBER!");
                println!();
                continue;
            }
        };

        // Compare the values
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!();
                println!("Too small!");
                println!();
            }
            Ordering::Greater => {
                println!();
                println!("Too big!");
                println!();
            }
            Ordering::Equal => {
                println!();
                println!("YOU WIN!");
                break;
            }
        }
    }
}
