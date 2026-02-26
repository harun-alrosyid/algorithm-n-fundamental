package main

import (
	"fmt"
	"strings"
)

// IsPalindrome checks if a string is a palindrome using the two-pointer approach.
// Time Complexity: O(n) - where n is the length of the string.
// Space Complexity: O(1) - as we are only using two pointers.
func IsPalindrome(s string) bool {
	// Normalize the string (lowercase for comparison)
	s = strings.ToLower(s)
	runes := []rune(s)

	for i, j := 0, len(runes)-1; i < j; i, j = i+1, j-1 {
		if runes[i] != runes[j] {
			return false
		}
	}
	return true
}

func main() {
	testCases := []string{"Racecar", "hello", "A man a plan a canal Panama"}

	for _, tc := range testCases {
		// Note: The simple version above considers spaces.
		// For "A man a plan a canal Panama" to be true, we'd need to strip spaces.
		// For this basic exercise, we'll keep it simple.
		fmt.Printf("Is '%s' a palindrome? %v\n", tc, IsPalindrome(tc))
	}
}
