package main

import(
	"log"
)

func change(data *[]int) {
	(*data)[0] = 1
	(*data)[1] = 2
	(*data)[2] = 3
	// *data = append(*data, 4, 5)
	*data = append(*data, 4, 5, 6)

}

func main() {
	data := make([]int, 3, 5)
	change(&data)
	log.Println(data, len(data), cap(data))
}