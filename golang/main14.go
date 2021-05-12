package main

import "fmt"

func main() {
	/*
	var x [5]float64
	x[0] = 98
	x[1] = 93
	x[2] = 77
	x[3] = 82
	x[4] = 83
	*/

	//x := [5]float64{98, 93, 77, 82, 83}
	x := [5]float64{
		98,
		93,
		77,
		82,
		//83
		83,
	}

	var total float64 = 0
	//for i := 0; i < 5; i++ {
	//for i := 0; i < len(x); i++ {
	//for i, value := range x {
	for _, value := range x {
		//total += x[i]
		total += value
	}

	//fmt.Println(total / 5)
	fmt.Println(total / float64(len(x)))
}