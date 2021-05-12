package main

import (
	"fmt"
)

type I interface {
	M()
}

type T struct {
	S string
}

/*
func (t T) M() {
	fmt.Println(t.S)
}
*/

func (t *T) M() {
	if (t == nil) {
		fmt.Println("<nil>")
		return
	}
	fmt.Println(t.S)
}

func describe(i I) {
	fmt.Printf("%v, %T\n", i, i)
}

func main() {
	var i I
	var t *T

	describe(i)
	//i.M()

	i = t
	describe(i)
	i.M()

	//i = T{"hzw"}
	//describe(i)
	//i.M()

	i = &T{"hzw"}
	describe(i)
	i.M()
}