package main

import (
	"fmt"
	"reflect"
)

func main() {
	r := 10 / 3
	fmt.Printf("%v\n", r)
	r1 := 10 / 3.
	fmt.Printf("%v\n", r1)
	r2 := 10 / float32(3)
	fmt.Printf("%v\n", r2)
	r3 := 10 / float64(3)
	fmt.Printf("%v\n", r3)

	fmt.Println(reflect.TypeOf(3.))
}
