/*
    when we add a semicolon at the end of an expression, it turns it into a statement, and statements do not return values.

    because statements do not evaluate to a value, is expressed by `()` the unit type. therefore nothing is returned 
*/
fn main() {
    println!("Hello Functions!");

    // Calling a function
    another_function();
    another_function_with_parameter(5);
    print_labeled_measurement(5, 'h');

    // Expressions and Statements
    let y = {
        let x = 3;
        x + 1
    };

    println!("y: {y}");

    // Functions with Return Values
    let x = five();
    println!("The value of x is: {x}");
    println!("The value of five() is: {}", five());

    let x = plus_one(5);
    println!("The value of x is: {x}");
    println!("The value of plus_one(5) is: {}", plus_one(5));
}

// Function Definitions
fn another_function() {
    println!("Another Function!");
}

fn another_function_with_parameter(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
