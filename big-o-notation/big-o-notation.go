package main

import "fmt"

// Big O(1)
var data = []int{1, 2, 3, 4, 5}

func getValueItem(n int) int {
	return data[n]
}

// Big O(n)
var data1 = []int{1, 2, 6, 4, 5, 9, 0, 3}

func getMaxValue(data []int) int {
	max := data[0]
	for _, val := range data {
		if val > max {
			max = val
		}
	}
	return max
}

func main() {

	fmt.Println(getValueItem(2))
	// output : 3
	fmt.Println(getValueItem(4))
	// output : 5

	// Big O(n)
	fmt.Println(getMaxValue(data1))
	// output : 9
}
