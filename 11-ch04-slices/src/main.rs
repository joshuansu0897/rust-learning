// A slice is borrowing, but only part of the data

/*
A slice points wherever the original data lives:
- `&s[0..5]` where s is a String: the data is on the heap.
- `&arr[1..3]` where arr is an array [i32; 5]: the data is on the stack. You did this in primitive_types4.
- `"hello"`` (a string literal) is already a &str: a slice of text stored inside your compiled program.

So a slice means “a view of part of some data”, wherever that data is.

Two details:
- It fails at compile time, not at runtime. The program never runs with a broken slice.
- It only fails if the slice is used after the change.
*/

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but s no longer has any content that we
    // could meaningfully use with the value 5, so word is now totally invalid!

    // ---

    let s = String::from("hello world");

    let _hello = &s[0..5];
    let _world = &s[6..11];

    // ---
    // let mut s = String::from("hello world");
    // let word = first_word_slice(&s); // word will get the value 5
    // s.clear(); // this empties the String, making it equal to ""
    // println!("The first word is: {word}");

    // ---

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole.
    let word = first_word_slice(&my_string[0..6]);
    let word = first_word_slice(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word = first_word_slice(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or
    // whole.
    let word = first_word_slice(&my_string_literal[0..6]);
    let word = first_word_slice(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_slice(my_string_literal);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
