package main

import "fmt"

func main() {
	// === Arrays (Slices in Go) ===

	names := []string{"Alice", "Bob", "Charlie", "Diana"}

	// O(1) Access
	fmt.Println("Array Access:", names[2])

	// O(n) Search
	searchArray := func(arr []string, target string) bool {
		for _, item := range arr {
			if item == target {
				return true
			}
		}
		return false
	}
	fmt.Println("Array Search:", searchArray(names, "Bob"))

	// === Hash Tables (Maps in Go) ===

	phoneBook := make(map[string]string)

	// O(1) Average - Insertion
	phoneBook["Alice"] = "555-0192"
	phoneBook["Bob"] = "555-8832"

	// O(1) Average - Access
	fmt.Println("Hash Table Access:", phoneBook["Alice"])


	// === Stacks (LIFO) ===
	
	stack := []int{}
	stack = append(stack, 10) // Push O(1)
	stack = append(stack, 20)

	fmt.Println("Stack Peek:", stack[len(stack)-1]) // Peek O(1)
	
	// Pop O(1)
	poppedValue := stack[len(stack)-1]
	stack = stack[:len(stack)-1]
	fmt.Println("Stack Pop:", poppedValue)


	// === Queues (FIFO) ===
	
	queue := []string{}
	queue = append(queue, "Alice") // Enqueue O(1)
	queue = append(queue, "Bob")

	fmt.Println("Queue Peek:", queue[0]) // Peek O(1)

	// Dequeue (In Go slices this reorganizes the underlying array or shifts pointers)
	dequeuedValue := queue[0]
	queue = queue[1:]
	fmt.Println("Queue Dequeue:", dequeuedValue)
}
