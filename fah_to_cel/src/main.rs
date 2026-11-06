fn main() {
    // Convert Celsius to Fahrenheit
    let celsius: f32 = 20.0;
    let fahrenheit: f32 = c_to_f(celsius);
    println!("Celsius ({celsius}) to Fahrenheit: {fahrenheit}");

    // Convert Fahrenheit  to Celsius
    let fahrenheit: f32 = 86.0;
    let celsius: f32 = f_to_c(fahrenheit);
    println!("Fahrenheit ({fahrenheit}) to Celsius: {celsius}");
}

fn c_to_f(celsius: f32) -> f32 {
    (celsius * 1.8) + 32.0
}

fn f_to_c(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) / 1.8
}
