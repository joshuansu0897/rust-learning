/*
    let and mut are different. with let we can change the type, is like recreate the variable;
    with mut we can change the value of the variable, but not the type

    in this block of code we are re-creating the variable immutable x with the same type but different value

    and we show the scope of the variable, in the inner scope we create a new variable x that is only available in that scope
    and when we leave the scope we go back to the previous value of x
 */
fn main() {
    let x;
    x = 5;
    println!("The value of x is: {x}");

    let x = x + 1;
    println!("The value of x is: {x}");

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}