package main

import "fmt"

func main() {
	fmt.Print("Enter a number: ")

	var in float64
	fmt.Scanf("%f", &in)

	out := in * 2

	fmt.Println(out)
}