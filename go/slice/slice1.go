package main

import (
	"log"
)

func change(data []int) {
	data[0] = 1
	data[1] = 2
	data[2] = 3
	// data = append(data, 4, 5)
	data = append(data, 4, 5, 6)
	// data = append(data, 4, 5, 6, 7)
	// data = append(data, 4, 5, 6, 7, 8)
	// data = append(data, 4, 5, 6, 7, 8, 9)
	// data = append(data, 4, 5, 6, 7, 8, 9, 10)
	// data = append(data, 4, 5, 6, 7, 8, 9, 10, 11)
	log.Println(len(data), cap(data))
}

func main() {
	// data := [5]int{}
	data := make([]int, 3, 5)
	// change(data[:3])
	change(data)
	log.Print(data)
	log.Print(data[:])
	log.Print(data[:cap(data)])
	log.Print(data)
}
