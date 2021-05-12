package main

import (
	"log"
)

func main() {
	a := []int{1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 1}
	log.Print(filter(a))
	log.Print(a[len(a):])
}

func filter(a []int) []int {
	l := len(a)
	for i := 0; i < l; i++ {
		if a[i] == 1 {
			copy(a[i:], a[i+1:])
			i -= 1
			l -= 1
		}
	}
	return a[:l]
}
