package main

import (
	"fmt"
)

func main() {
	a := []int{1, 2, 3, 4, 5}
	b := a[1:3];

	fmt.Printf("a: len: %d, cap: %d\n", len(a), cap(a))
	fmt.Println("a:", a)
	fmt.Printf("b: len: %d, cap: %d\n", len(b), cap(b))
	fmt.Println("b:", b)

	fmt.Println()
	b = append(b, 6)
	fmt.Printf("a: len: %d, cap: %d\n", len(a), cap(a))
	fmt.Println("a:", a)
	fmt.Printf("len: %d, cap: %d\n", len(b), cap(b))
	fmt.Println("b:", b)

	fmt.Println()
	b[0] = 7
	fmt.Printf("a: len: %d, cap: %d\n", len(a), cap(a))
	fmt.Println("a:", a)
	fmt.Printf("len: %d, cap: %d\n", len(b), cap(b))
	fmt.Println("b:", b)

	fmt.Println()
	b = append(b, 8)
	fmt.Printf("a: len: %d, cap: %d\n", len(a), cap(a))
	fmt.Println("a:", a)
	fmt.Printf("len: %d, cap: %d\n", len(b), cap(b))
	fmt.Println("b:", b)

	fmt.Println()
	b[0] = 9
	fmt.Printf("a: len: %d, cap: %d\n", len(a), cap(a))
	fmt.Println("a:", a)
	fmt.Printf("len: %d, cap: %d\n", len(b), cap(b))
	fmt.Println("b:", b)

	fmt.Println()
	b = append(b, 10)
	fmt.Printf("a: len: %d, cap: %d\n", len(a), cap(a))
	fmt.Println("a:", a)
	fmt.Printf("len: %d, cap: %d\n", len(b), cap(b))
	fmt.Println("b:", b)
	
	fmt.Println()
	b[0] = 11;
	fmt.Printf("a: len: %d, cap: %d\n", len(a), cap(a))
	fmt.Println("a:", a)
	fmt.Printf("len: %d, cap: %d\n", len(b), cap(b))
	fmt.Println("b:", b)

	fmt.Println()
	b = a[1:3]
	b = append(b, 10, 11, 12)
	fmt.Printf("a: len: %d, cap: %d\n", len(a), cap(a))
	fmt.Println("a:", a)
	fmt.Printf("len: %d, cap: %d\n", len(b), cap(b))
	fmt.Println("b:", b)
}