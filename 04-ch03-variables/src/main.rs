// Shadowing vs `mut`:
// - `let x = ...` again creates a NEW variable with the same name
//   (shadowing). The new one can even have a different type.
// - `let mut x` lets you change the value of the SAME variable, but its type
//   can never change.
// - A shadow created inside a `{ }` block only lives until the block ends;
//   after it, the outer `x` is visible again.

// Constants: always UPPER_SNAKE_CASE, always with a type, and the value must
// be known at compile time.
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // WHY: clippy flags `let x; x = 5;` ("unneeded late initialization").
    // Declaring and assigning in one line is clearer. Late initialization is
    // only useful when the value depends on a branch, e.g.
    // `let x; if cond { x = 1 } else { x = 2 }` (and even then, `if` as an
    // expression is usually nicer).
    let x = 5;
    println!("The value of x is: {x}");

    let x = x + 1;
    println!("The value of x is: {x}");

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // Shadowing can change the type: a string becomes a number.
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Number of spaces: {spaces}");

    // `mut` cannot change the type. Uncomment to see error E0308:
    // let mut spaces = "   ";
    // spaces = spaces.len();

    println!("Three hours = {THREE_HOURS_IN_SECONDS} seconds");
}
