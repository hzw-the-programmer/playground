package main

import (
	"fmt"
)

func main() {
	serve()
	for {
	}
}

func serve() {
	done := make(chan struct{})
	defer close(done)
	c := sq(done, sq(done, gen(done, 2, 3, 4, 5)))
	fmt.Println(<-c)
}

func gen(done <-chan struct{}, nums ...int) <-chan int {
	out := make(chan int)

	go func() {
		defer fmt.Println("finish gen")
		defer close(out)
		for _, num := range nums {
			select {
			case out <- num:
			case <-done:
				return
			}
		}
	}()

	return out
}

func sq(done <-chan struct{}, in <-chan int) <-chan int {
	out := make(chan int)

	go func() {
		defer fmt.Println("finish sq")
		defer close(out)
		for num := range in {
			select {
			case out <- num * num:
			case <-done:
				return
			}
		}
	}()

	return out
}
