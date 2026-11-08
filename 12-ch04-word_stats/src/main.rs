// Standard
use std::io;

fn main() {
    // Tittle
    println!("##############################");
    println!("#         Word Stats!        #");
    println!("##############################");
    println!();

    // Request user input
    loop {
        println!("Please input your phrase: ");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Clean the input
        let input = input.trim();

        if input.is_empty() {
            println!();
            println!("TYPE A PHRASE!");
            println!();
            continue;
        }

        let first = first_word(input);
        let longest = longest_word(input);
        let word_count: usize = word_count(input);

        println!();
        println!("first:'{first}', longest:'{longest}', how many words:'{word_count}'");
        println!();
    }
}

fn first_word(s: &str) -> &str {
    let str_bytes = s.as_bytes();

    for (index, &item) in str_bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index];
        }
    }

    s
}

fn longest_word(s: &str) -> &str {
    let str_bytes = s.as_bytes();

    let mut start_index = 0;
    let mut end_index = str_bytes.len();

    let mut acc = 0;
    let mut local_acc = 0;
    let mut last_was_space = false;
    for (index, &item) in str_bytes.iter().enumerate() {
        if item == b' ' && !last_was_space {
            if local_acc > acc {
                end_index = index;
                start_index = index - local_acc;
                acc = local_acc;
            }

            local_acc = 0;
            last_was_space = true;
        } else if item == b' ' && last_was_space {
            continue;
        } else {
            last_was_space = false;
            local_acc += 1;
        }
    }

    if local_acc > acc {
        let last_index = str_bytes.len() - local_acc;
        return &s[last_index..];
    }

    &s[start_index..end_index]
}

fn word_count(s: &str) -> usize {
    let str_bytes = s.as_bytes();

    let mut acc = 0;
    let mut last_was_space = false;
    for &item in str_bytes.iter() {
        if item == b' ' && !last_was_space {
            acc += 1;

            last_was_space = true;
        } else if item == b' ' && last_was_space {
            continue;
        } else {
            last_was_space = false;
        }
    }

    acc += 1;
    acc
}
