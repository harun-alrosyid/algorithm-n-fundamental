package main

import "fmt"

// Big O(1)
var data = []int{1, 2, 3, 4, 5}

func getValueItem(n int) int {
	return data[n]
}

func main() {

	fmt.Println(getValueItem(2))
	// output : 3
	fmt.Println(getValueItem(4))
	// output : 5
}
