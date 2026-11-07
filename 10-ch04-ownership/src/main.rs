// the known sizes types goes into the `stack` and popped off the `stack` when their scope is over


fn main() { // `s` is not valid here since it's not yet declared
    let _s = "string"; // `s` is valid from this point forward / this is a string literal, where the string value is hardcoded into our program / son immutables

    // do stuff with `s`

    let mut s2 = String::from("hello"); // this is a `heap` variable/value / this can be mutated

    s2.push_str(", world!"); 
    println!("{s2}");

    // the difference between the literal and the String::from is in how these two types deal with memory

    // ----

    // in this case bind the value 5 to x and make a copy of the value in x and bind it to y; because we have fixed sizes, goes to the `stack`
    let x = 5;
    let _y = x;

    // ---
    // A String is made up of three parts, shown on the left: a pointer to the memory that holds the contents of the string, a length, and a capacity. 
    // This group of data is stored on the stack
    let s1 = String::from("hello");
    // When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack
    let _s2 = s1;

    // if we remember Rust calls `drop` function at the closing curl brackets, this means right now we have two references to the same data in the heap
    // will try to free double the same data, that is a issue called `double free`
    // Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities

    // Rust fix it, if we try to use the `s1` like `println!("{s1}, world!");` will fail with `error[E0382]: borrow of moved value: `s1``
    // now just s2 had valid references to the data in the heap `one single owner`

    // in this block of code rust call `drop` when we assign a new value to an existing variable
    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!");

    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);             // s's value moves into the function...
    //println!("{s}, world!");        // THIS WILL FAIL BECAUSE THE `s` OWNERSHIP GOES TO THE FUNCTION `s` IS NOT VALID HERE 
                                    // ... and so is no longer valid here
    let x = 5;                      // x comes into scope
    makes_copy(x);                  // Because i32 implements the Copy trait,
                                    // x does NOT move into the function,
                                    // so it's okay to use x afterward.


    let _s1 = gives_ownership();       // gives_ownership moves its return
                                       // value into s1

    let s2 = String::from("hello");    // s2 comes into scope
    let _s3 = takes_and_gives_back(s2);// s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{s2}' is {len}.");

} // this scope is now over, and `s` is not longer valid same with `s2`

// when a variable goes out of scope Rust calls a function called `drop`; Rust calls `drop` automatically at the closing curly bracket

// NOTE: C++ have the pattern Resource Acquisition Is Initialization (RAII). The drop function in Rust will be familiar to you if you’ve used RAII patterns.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {       // gives_ownership will move its
                                       // return value into the function
                                       // that calls it

    let some_string = String::from("yours"); // some_string comes into scope
    some_string                        // some_string is returned and
                                       // moves out to the calling
                                       // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope
    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}