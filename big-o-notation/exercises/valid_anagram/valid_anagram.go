package main

import "fmt"

/**
 * Problem: Valid Anagram
 * Complexity:
 * - Time: O(n)
 * - Space: O(1)
 */

func isAnagram(s string, t string) bool {
	if len(s) != len(t) {
		return false
	}

	count := make(map[rune]int)

	for _, char := range s {
		count[char]++
	}

	for _, char := range t {
		count[char]--
		if count[char] < 0 {
			return false
		}
	}

	return true
}

func main() {
	s1, t1 := "anagram", "nagaram"
	s2, t2 := "rat", "car"

	fmt.Printf("s1: \"%s\", t1: \"%s\" -> Result: %v\n", s1, t1, isAnagram(s1, t1))
	fmt.Printf("s2: \"%s\", t2: \"%s\" -> Result: %v\n", s2, t2, isAnagram(s2, t2))

	if isAnagram(s1, t1) == true && isAnagram(s2, t2) == false {
		fmt.Println("Test Passed! ✅")
	} else {
		fmt.Println("Test Failed! ❌")
	}
}
