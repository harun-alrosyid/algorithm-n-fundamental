/**
 * Factorial & Fibonacci Analysis
 * 
 * Factorial:
 * - Recursive: O(n) time, O(n) space (stack)
 * - Iterative: O(n) time, O(1) space
 * 
 * Fibonacci:
 * - Recursive: O(2^n) time (without memo), O(n) space
 * - Iterative: O(n) time, O(1) space
 */

// --- Factorial ---

function factorialRecursive(n: number): number {
    if (n <= 1) return 1; // Base case
    return n * factorialRecursive(n - 1); // Recursive step
}

function factorialIterative(n: number): number {
    let result = 1;
    for (let i = 2; i <= n; i++) {
        result *= i;
    }
    return result;
}

// --- Fibonacci ---

function fibonacciRecursive(n: number): number {
    if (n <= 1) return n;
    return fibonacciRecursive(n - 1) + fibonacciRecursive(n - 2);
}

function fibonacciIterative(n: number): number {
    if (n <= 1) return n;
    let prev = 0, curr = 1;
    for (let i = 2; i <= n; i++) {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    return curr;
}

// --- Demo ---

const nFactorial = 5;
const nFibonacci = 10;

console.log(`--- Factorial of ${nFactorial} ---`);
console.log(`Recursive: ${factorialRecursive(nFactorial)}`);
console.log(`Iterative: ${factorialIterative(nFactorial)}`);

console.log(`\n--- Fibonacci of ${nFibonacci} ---`);
console.log(`Recursive: ${fibonacciRecursive(nFibonacci)}`);
console.log(`Iterative: ${fibonacciIterative(nFibonacci)}`);
