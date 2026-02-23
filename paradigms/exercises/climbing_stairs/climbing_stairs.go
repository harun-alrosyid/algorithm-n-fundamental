package main

import "fmt"

/**
 * Problem: Climbing Stairs
 */

func climbStairs(n int) int {
	if n <= 2 {
		return n
	}

	oneStepBefore := 2
	twoStepsBefore := 1
	currentWays := 0

	for i := 3; i <= n; i++ {
		currentWays = oneStepBefore + twoStepsBefore
		twoStepsBefore = oneStepBefore
		oneStepBefore = currentWays
	}

	return currentWays
}

func main() {
	testN := 5
	result := climbStairs(testN)

	fmt.Printf("Steps: %d, Distinct ways: %d\n", testN, result)

	if climbStairs(2) == 2 && climbStairs(3) == 3 && result == 8 {
		fmt.Println("Test Passed! ✅")
	} else {
		fmt.Println("Test Failed! ❌")
	}
}
