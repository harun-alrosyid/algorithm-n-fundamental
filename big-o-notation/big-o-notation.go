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

// Big O(log n)
func firstPriceGte(prices []int, target int) int {
	lo := 0
	hi := len(prices) - 1
	ans := -1
	for lo <= hi {
		mid := (lo + hi) / 2
		if prices[mid] >= target {
			ans = mid
			hi = mid - 1
		} else {
			lo = mid + 1
		}
	}
	if ans == -1 {
		return 0
	}
	return prices[ans]

}

func main() {

	fmt.Println(getValueItem(2))
	// output : 3
	fmt.Println(getValueItem(4))
	// output : 5

	// Big O(n)
	fmt.Println(getMaxValue(data1))
	// output : 9

	priceList := []int{20_000, 45_000, 60_000, 90_000, 120_000, 160_000, 200_000}
	thresholdFreeDelivery := 150_000
	i := firstPriceGte(priceList, thresholdFreeDelivery)
	fmt.Println(i)
	// output : 160000
}
