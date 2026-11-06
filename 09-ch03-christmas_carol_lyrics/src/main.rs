// The point of this exercise is to *generate* the song with loops instead of
// copying every verse. Each verse repeats the previous gifts, so the data
// lives in arrays and the code only describes the pattern.
//
// `&str` is a string slice (Chapter 4.3). A `const` array of them is data
// that is fixed at compile time.
const ORDINALS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

// Index 0 is the partridge; it's printed separately because its first word
// changes ("A" on day 1, "And" afterwards).
const GIFTS: [&str; 12] = [
    "partridge in a pear tree.",
    "Two turtle doves,",
    "Three French hens,",
    "Four calling birds,",
    "Five golden rings,",
    "Six geese a-laying,",
    "Seven swans a-swimming,",
    "Eight maids a-milking,",
    "Nine ladies dancing,",
    "Ten lords a-leaping,",
    "Eleven pipers piping,",
    "Twelve drummers drumming,",
];

fn main() {
    // WHY: `.iter().enumerate()` gives the index *and* the value, so you don't
    // index the array by hand (Chapter 13 covers iterators).
    for (day, ordinal) in ORDINALS.iter().enumerate() {
        print_verse(day, ordinal);
    }
}

fn print_verse(day: usize, ordinal: &str) {
    println!("On the {ordinal} day of Christmas");
    println!("My true love gave to me");

    // Count down from today's gift to the second one.
    // `GIFTS[1..=day]` is a slice of the array (Chapter 4.3); `.rev()` walks
    // it backwards, like `(1..4).rev()` in Chapter 3.5.
    for gift in GIFTS[1..=day].iter().rev() {
        println!("{gift}");
    }

    // WHY: `if` is an expression (Chapter 3.5), so it can choose a value.
    let article = if day == 0 { "A" } else { "And a" };
    println!("{article} {}", GIFTS[0]);
    println!();
}
