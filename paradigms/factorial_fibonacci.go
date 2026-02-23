package main

import "fmt"

/**
 * Recursion vs Iteration in Go
 */

// --- Factorial ---

func factorialRecursive(n int) int {
	if n <= 1 {
		return 1
	}
	return n * factorialRecursive(n-1)
}

func factorialIterative(n int) int {
	result := 1
	for i := 2; i <= n; i++ {
		result *= i
	}
	return result
}

// --- Fibonacci ---

func fibonacciRecursive(n int) int {
	if n <= 1 {
		return n
	}
	return fibonacciRecursive(n-1) + fibonacciRecursive(n-2)
}

func fibonacciIterative(n int) int {
	if n <= 1 {
		return n
	}
	prev, curr := 0, 1
	for i := 2; i <= n; i++ {
		prev, curr = curr, prev+curr
	}
	return curr
}

func main() {
	nFactorial := 5
	nFibonacci := 10

	fmt.Printf("--- Factorial of %d ---\n", nFactorial)
	fmt.Printf("Recursive: %d\n", factorialRecursive(nFactorial))
	fmt.Printf("Iterative: %d\n", factorialIterative(nFactorial))

	fmt.Printf("\n--- Fibonacci of %d ---\n", nFibonacci)
	fmt.Printf("Recursive: %d\n", fibonacciRecursive(nFibonacci))
	fmt.Printf("Iterative: %d\n", fibonacciIterative(nFibonacci))
}
