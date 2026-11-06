fn main() {
    let celsius = 20.0;
    println!("{celsius} °C = {} °F", c_to_f(celsius));

    let fahrenheit = 86.0;
    println!("{fahrenheit} °F = {} °C", f_to_c(fahrenheit));

    // WHY: a small table shows the conversion on several values at once.
    // `.step_by(20)` is an iterator method (Chapter 13).
    println!();
    for celsius in (0..=100).step_by(20) {
        let celsius = f64::from(celsius); // shadowing: i32 -> f64
        println!("{celsius:>5} °C = {:>5} °F", c_to_f(celsius));
    }
}

// WHY f64 instead of f32: f64 is Rust's default float type, more precise, and
// on modern CPUs about as fast. Use f32 only when you need to save memory.
//
// WHY `9.0 / 5.0` instead of `1.8`: it reads like the formula you know, and
// multiplying before dividing keeps the common values exact
// (212 °F gives exactly 100 °C).
fn c_to_f(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn f_to_c(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

// Tests run with `cargo test` (Chapter 11).
#[cfg(test)]
mod tests {
    use super::*;

    // WHY: never compare floats with `==`. Rounding errors make
    // 0.1 + 0.2 == 0.3 false. Compare with a small tolerance instead.
    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 1e-9,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn known_points() {
        assert_close(c_to_f(0.0), 32.0);
        assert_close(c_to_f(100.0), 212.0);
        assert_close(f_to_c(212.0), 100.0);
        assert_close(f_to_c(-40.0), -40.0); // the scales cross at -40
    }

    #[test]
    fn round_trip() {
        assert_close(f_to_c(c_to_f(37.0)), 37.0);
    }
}
