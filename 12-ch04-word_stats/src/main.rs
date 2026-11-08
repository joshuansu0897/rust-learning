use std::io;

fn main() {
    println!("##############################");
    println!("#         Word Stats!        #");
    println!("##############################");
    println!("Type a phrase. To quit: Ctrl+Z then Enter (Windows) or Ctrl+D.\n");

    // WHY one buffer outside the loop: `String::new()` inside the loop allocates
    // a new String on every line. Reusing one and calling `clear()` keeps the
    // same heap memory. `clear()` needs `&mut input`, and that's only allowed
    // because `line` (a `&str` borrowed from `input`) is no longer used by then:
    // the borrowing rules from 4.2 at work.
    let mut input = String::new();

    loop {
        println!("Please input your phrase:");
        input.clear();

        let bytes_read = io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // WHY: `read_line` returns 0 when the input ends (Ctrl+Z / Ctrl+D).
        // Without this check the loop never ends: the same bug the guessing
        // game had.
        if bytes_read == 0 {
            println!("\nBye!");
            break;
        }

        let line = input.trim();

        if line.is_empty() {
            println!("\nTYPE A PHRASE!\n");
            continue;
        }

        // WHY: no variable named `word_count`, so it doesn't shadow the
        // function with the same name.
        let first = first_word(line);
        let longest = longest_word(line);
        let count = word_count(line);

        println!("\nfirst: '{first}', longest: '{longest}', how many words: {count}\n");
    }
}

// WHY `is_ascii_whitespace()` instead of `== b' '`: tabs and other spaces also
// separate words, so "hi\tthere" is two words.
//
// WHY slicing by byte index is safe here: we only cut at ASCII whitespace
// bytes, and those never appear inside a multi-byte character like `ñ`. So
// `&s[..i]` never splits a character in half (that would panic).

/// Returns the first word of `s`, or all of `s` if it has no whitespace.
fn first_word(s: &str) -> &str {
    for (i, &byte) in s.as_bytes().iter().enumerate() {
        if byte.is_ascii_whitespace() {
            return &s[..i];
        }
    }

    s
}

/// Returns the longest word in `s`, measured in bytes. On a tie, the first
/// one wins. Returns `""` if `s` has no words.
fn longest_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    // WHY keep a slice instead of `acc`, `start_index` and `end_index`: the
    // slice already knows its start and its length (`longest.len()`), so
    // there's nothing to compute by hand.
    let mut longest = "";

    // WHY remember where the word *starts*: when the word ends at `i`, the word
    // is simply `&s[word_start..i]`. No counter, no `index - local_acc`, so the
    // off-by-one from bug 3 can't happen.
    let mut word_start = 0;
    let mut in_word = false;

    // WHY `0..=bytes.len()` (one step past the end): that extra position acts
    // as one more separator, so the last word is checked inside the loop and
    // doesn't need a second copy of the comparison after it (bug 2).
    for i in 0..=bytes.len() {
        // `||` stops early: when `i == bytes.len()` it never reads `bytes[i]`,
        // which would be out of bounds.
        let at_separator = i == bytes.len() || bytes[i].is_ascii_whitespace();

        if at_separator {
            if in_word {
                let word = &s[word_start..i];
                // `>` (not `>=`) keeps the first word on a tie.
                if word.len() > longest.len() {
                    longest = word;
                }
                in_word = false;
            }
        } else if !in_word {
            word_start = i;
            in_word = true;
        }
    }

    longest
}

/// Counts the words in `s`. Several spaces in a row count as one separator.
fn word_count(s: &str) -> usize {
    // WHY count the *starts* of words: a word starts at a letter that comes
    // right after a separator (or at the very beginning). No `+ 1` at the end,
    // and an empty string correctly gives 0.
    let mut count = 0;
    let mut in_word = false;

    for &byte in s.as_bytes() {
        if byte.is_ascii_whitespace() {
            in_word = false;
        } else if !in_word {
            count += 1;
            in_word = true;
        }
    }

    count
}

// Run with `cargo test` (Chapter 11). Every bug you fixed by hand has a test
// here, so it can't come back.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_word_cases() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("hello"), "hello");
        assert_eq!(first_word("hi\tthere"), "hi");
    }

    #[test]
    fn longest_word_cases() {
        assert_eq!(longest_word("hello"), "hello");
        assert_eq!(longest_word("hi hello"), "hello"); // bug 2: last word
        assert_eq!(longest_word("a bb ccc"), "ccc");
        assert_eq!(longest_word("one two three four"), "three");
        assert_eq!(longest_word("aa  bbbb c"), "bbbb"); // bug 3: no leading space
        assert_eq!(longest_word("ab cd ef"), "ab"); // tie: first one wins
        assert_eq!(longest_word("hi\tthere"), "there");
        assert_eq!(longest_word(""), "");
    }

    #[test]
    fn word_count_cases() {
        assert_eq!(word_count("hello"), 1);
        assert_eq!(word_count("one two three four"), 4);
        assert_eq!(word_count("aa   b"), 2);
        assert_eq!(word_count("hi\tthere friend"), 3);
        assert_eq!(word_count(""), 0);
    }

    #[test]
    fn non_ascii_letters_dont_panic() {
        assert_eq!(first_word("ñandú es"), "ñandú");
        assert_eq!(longest_word("ñandú es"), "ñandú"); // 7 bytes, not 5 letters
    }
}
