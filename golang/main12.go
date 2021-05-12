package main

import "fmt"

func main() {
	/*
	fmt.Println(1)
	fmt.Println(2)
	fmt.Println(3)
	fmt.Println(4)
	fmt.Println(5)
	fmt.Println(6)
	fmt.Println(7)
	fmt.Println(8)
	fmt.Println(9)
	fmt.Println(10)
	*/

	/*
	fmt.Println(`1
2
3
4
5
6
7
8
9
10`)
*/

	//fmt.Println("1\n2\n3\n4\n5\n6\n7\n8\n9\n10")

	/*
	i := 1
	for i <= 10 {
		fmt.Println(i)
		i = i + 1
	}
	*/

	/*
	for i := 1; i <=10; i++ {
		fmt.Println(i)
	}
	*/

	/*
	for i := 1; i <= 10; i++ {
		if i % 2 == 0 {
			fmt.Println(i, "even")
		} else {
			fmt.Println(i, "old")
		}
	}
	*/

	var i int
	fmt.Scanf("%d", &i)
	switch i {
	case 0: fmt.Println("Zero")
	case 1: fmt.Println("One")
	case 2: fmt.Println("Two")
	case 3: fmt.Println("Three")
	case 4: fmt.Println("Four")
	case 5: fmt.Println("Five")
	default: fmt.Println("Unknow Number")
	}
}