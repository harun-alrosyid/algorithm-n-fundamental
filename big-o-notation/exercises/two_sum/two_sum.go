package main

import "fmt"

/**
 * Problem: Two Sum
 * Complexity:
 * - Time: O(n)
 * - Space: O(n)
 *
 * Memory Model:
 * - In Go, maps are reference types. When you pass a map to a function,
 *   you're passing a pointer to the underlying data structure in the Heap.
 * - Slices (like []int) are also reference-like, pointing to an array in the Heap.
 */

func twoSum(nums []int, target int) []int {
	prevMap := make(map[int]int)

	for i, num := range nums {
		diff := target - num
		if idx, ok := prevMap[diff]; ok {
			return []int{idx, i}
		}
		prevMap[num] = i
	}

	return nil
}

func main() {
	nums := []int{2, 7, 11, 15}
	target := 9
	result := twoSum(nums, target)

	fmt.Printf("Input: nums = %v, target = %d\n", nums, target)
	fmt.Printf("Output: %v\n", result)

	if len(result) == 2 && result[0] == 0 && result[1] == 1 {
		fmt.Println("Test Passed! ✅")
	} else {
		fmt.Println("Test Failed! ❌")
	}
}
