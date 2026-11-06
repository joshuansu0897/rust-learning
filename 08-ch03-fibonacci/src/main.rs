// The biggest n whose Fibonacci number still fits in a u64.
// fib(93) = 12_200_160_415_121_876_738; fib(94) is bigger than u64::MAX.
const MAX_N: u32 = 93;

fn main() {
    // WHY: one loop instead of 11 copied println! lines.
    for n in 0..=10 {
        println!("The {n} fibonacci is: {}", fibonacci(n));
    }

    println!("The {MAX_N} fibonacci is: {}", fibonacci(MAX_N));
}

/// Returns the nth Fibonacci number: fib(0) = 0, fib(1) = 1.
///
/// Works for n up to MAX_N (93). Beyond that the result does not fit in a u64,
/// so a debug build panics with "attempt to add with overflow".
//
// WHY these types:
// - `n: u32`: negative positions don't exist, so let the type forbid them.
//   Your `if n <= 0` check disappears: the compiler does it for you.
// - `-> u64`: an i32 overflows at fib(47); a u64 reaches fib(93).
fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }

    // WHY: a tuple keeps the pair together, and the tuple assignment below
    // updates both at once, so the `tmp` variable is no longer needed.
    let (mut previous, mut current): (u64, u64) = (0, 1);

    // WHY: `for` over a range instead of `while n > 1 { ... n -= 1 }`.
    // No counter to mutate, no off-by-one risk. `_` means "I don't need the
    // loop variable".
    for _ in 1..n {
        (previous, current) = (current, previous + current);
    }

    // WHY: the last expression is the return value. No `return`, no `;`.
    current
}

// Tests run with `cargo test`. You'll learn them in Chapter 11; for now,
// notice how they document what the function promises.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_numbers() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn biggest_that_fits_in_u64() {
        assert_eq!(fibonacci(MAX_N), 12_200_160_415_121_876_738);
    }
}
