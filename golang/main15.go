package main

import "fmt"

func main() {
	x := make([]float64, 5)
	fmt.Println(x)
	//x := make([]float64, 5, 10)
	x = make([]float64, 6, 10)
	fmt.Println(x)
	a := []float64{1, 2, 3, 4, 5, 6, 7}
	//x = a[0:7]
	x = a[0:len(a)]
	//x = a[:7]
	//x = a[0:]
	//x = a[:]
	fmt.Println(x)
}