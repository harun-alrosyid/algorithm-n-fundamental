/**
 * Recursion vs Iteration in Rust
 */

// --- Factorial ---

fn factorial_recursive(n: u64) -> u64 {
    if n <= 1 {
        return 1;
    }
    n * factorial_recursive(n - 1)
}

fn factorial_iterative(n: u64) -> u64 {
    let mut result = 1;
    for i in 2..=n {
        result *= i;
    }
    result
}

// --- Fibonacci ---

fn fibonacci_recursive(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}

fn fibonacci_iterative(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    let mut prev = 0;
    let mut curr = 1;
    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    curr
}

fn main() {
    let n_factorial = 5;
    let n_fibonacci = 10;

    println!("--- Factorial of {} ---", n_factorial);
    println!("Recursive: {}", factorial_recursive(n_factorial));
    println!("Iterative: {}", factorial_iterative(n_factorial));

    println!("\n--- Fibonacci of {} ---", n_fibonacci);
    println!("Recursive: {}", fibonacci_recursive(n_fibonacci));
    println!("Iterative: {}", fibonacci_iterative(n_fibonacci));
}
