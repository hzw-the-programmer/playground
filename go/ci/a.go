package main

import (
	"log"
)

func CloudmallInterview1(numbers []int) []int {
	i := 0
	j := len(numbers) - 1
	
	if j <= 0 {
		return numbers
	}

	var t int
	for i < j {
		for numbers[i] >= 0 {
			i++
		}
		for numbers[j] < 0 {
			j--
		}
		if i < j {
			t = numbers[i]
			numbers[i] = numbers[j]
			numbers[j] = t
			i++
			j++
		}
	}

	i = 0
	for i < j {
		for numbers[i] > 0 {
			i++
		}
		for numbers[j] == 0 {
			j--
		}
		if i < j {
			t = numbers[i]
			numbers[i] = numbers[j]
			numbers[j] = t
			i++
			j++
		}
	}

	return numbers
}

func main() {
	a := []int{6, 4, -3, 0, 5, -2, -1, 0, 1, -9}
	CloudmallInterview1(a)
	log.Print(a)
}
