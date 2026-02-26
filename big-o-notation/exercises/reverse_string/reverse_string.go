package main

import (
	"fmt"
)

// ReverseString reverses a string using a rune slice for Unicode safety.
// Time Complexity: O(n) - where n is the number of characters in the string.
// Space Complexity: O(n) - to store the rune slice.
func ReverseString(s string) string {
	runes := []rune(s)
	for i, j := 0, len(runes)-1; i < j; i, j = i+1, j-1 {
		runes[i], runes[j] = runes[j], runes[i]
	}
	return string(runes)
}

func main() {
	input := "hello world"
	reversed := ReverseString(input)
	fmt.Printf("Original: %s\n", input)
	fmt.Printf("Reversed: %s\n", reversed)
}
