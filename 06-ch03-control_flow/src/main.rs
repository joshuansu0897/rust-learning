// Rustaceans prefer `for` over `while` with an index: there's no counter to
// update, so you can't go out of bounds or forget an element.
fn main() {
    // Just a simple if statement
    let number = 5;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    // `if` is an expression, which means it evaluates to a value. We can use that value to assign a variable.
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // Returning values from loops
    // You can also `return` from inside a loop. While `break` only exits the current loop, `return` always exits the current function.
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");

    println!();

    // Disambiguating with loop labels
    let mut count = 0;

    'counting_up: loop {
        println!("Count: {count}");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End Count: {count}");
    println!();

    // Streamlining Conditional Loops with While
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("{number}");
    println!();

    // Looping Through a Collection with while
    // WHY the book shows this: it works, but it's fragile. Change the array to
    // 4 elements and `array[index]` panics at runtime. The `for` below can't.
    let array = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", array[index]);
        index += 1;
    }
    println!();

    // Using For
    let array = [10, 20, 30, 40, 50];

    for item in array {
        println!("The value is: {item}");
    }
    println!();

    // For with Range
    for number in (1..10).rev() {
        println!("{number}");
    }
}
