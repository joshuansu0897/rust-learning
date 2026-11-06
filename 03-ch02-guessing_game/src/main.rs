use std::cmp::Ordering;
use std::io;

// WHY: named constants instead of "magic numbers". If you change the range,
// you change it in one place, and the prompt below stays correct.
const MIN: u32 = 1;
const MAX: u32 = 100;

fn main() {
    println!("##############################");
    println!("#      Guess the number!     #");
    println!("##############################");
    println!();

    // WHY: rand 0.10 has a free function, so no trait import is needed.
    // `use rand::prelude::*` imported many names you never used; explicit
    // code is easier to read and review.
    let secret_number: u32 = rand::random_range(MIN..=MAX);
    let mut attempts = 0;

    loop {
        println!("Please input your guess ({MIN}-{MAX}):");
        let mut guess = String::new();

        // WHY: `read_line` returns how many bytes it read. 0 means the input
        // ended (Ctrl+Z on Windows, Ctrl+D on Linux/macOS, or a closed pipe).
        // Your version ignored this, so it looped forever printing
        // "TYPE A NUMBER!". Always ask: "what happens when the input ends?"
        let bytes_read = io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        if bytes_read == 0 {
            println!("\nNo more input. The number was {secret_number}.");
            return;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                // WHY: "\n" inside the string replaces three println! calls.
                println!("\nTYPE A NUMBER!\n");
                continue;
            }
        };

        // WHY: validate the range too. Before, 500 was accepted and just
        // reported "Too big!". Treat every input as untrusted.
        if !(MIN..=MAX).contains(&guess) {
            println!("\nThe number is between {MIN} and {MAX}.\n");
            continue;
        }

        attempts += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("\nToo small!\n"),
            Ordering::Greater => println!("\nToo big!\n"),
            Ordering::Equal => {
                println!("\nYOU WIN in {attempts} attempts!");
                break;
            }
        }
    }
}
