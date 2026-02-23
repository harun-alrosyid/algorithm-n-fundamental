"""
Recursion vs Iteration in Python
"""

# --- Factorial ---

def factorial_recursive(n: int) -> int:
    if n <= 1:
        return 1
    return n * factorial_recursive(n - 1)

def factorial_iterative(n: int) -> int:
    result = 1
    for i in range(2, n + 1):
        result *= i
    return result

# --- Fibonacci ---

def fibonacci_recursive(n: int) -> int:
    if n <= 1:
        return n
    return fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)

def fibonacci_iterative(n: int) -> int:
    if n <= 1:
        return n
    prev, curr = 0, 1
    for _ in range(2, n + 1):
        prev, curr = curr, prev + curr
    return curr

if __name__ == "__main__":
    n_factorial = 5
    n_fibonacci = 10

    print(f"--- Factorial of {n_factorial} ---")
    print(f"Recursive: {factorial_recursive(n_factorial)}")
    print(f"Iterative: {factorial_iterative(n_factorial)}")

    print(f"\n--- Fibonacci of {n_fibonacci} ---")
    print(f"Recursive: {fibonacci_recursive(n_fibonacci)}")
    print(f"Iterative: {fibonacci_iterative(n_fibonacci)}")
