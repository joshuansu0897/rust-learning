fn main() {
    println!("The {} fibonacci is: {}", 0, fibonacci(0));
    println!("The {} fibonacci is: {}", 1, fibonacci(1));
    println!("The {} fibonacci is: {}", 2, fibonacci(2));
    println!("The {} fibonacci is: {}", 3, fibonacci(3));
    println!("The {} fibonacci is: {}", 4, fibonacci(4));
    println!("The {} fibonacci is: {}", 5, fibonacci(5));
    println!("The {} fibonacci is: {}", 6, fibonacci(6));
    println!("The {} fibonacci is: {}", 7, fibonacci(7));
    println!("The {} fibonacci is: {}", 8, fibonacci(8));
    println!("The {} fibonacci is: {}", 9, fibonacci(9));
    println!("The {} fibonacci is: {}", 10, fibonacci(10));
}

fn fibonacci(mut n: i32) -> i32 {

    if n <= 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    let mut num1 = 0;
    let mut num2 = 1;

    let mut tmp = 0;

    while n > 1 {
        tmp = num1 + num2;
        num1 = num2;
        num2 = tmp;
        n -= 1;
    }

    return tmp;
}
