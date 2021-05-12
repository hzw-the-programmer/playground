package main

import (
	"fmt"
	"runtime"
	"sync"
)

func func2() {
	defer func() {
		fmt.Println("func2 defer")
	}()
	fmt.Println("func2")
	runtime.Goexit()
}

func func3() {
	defer func() {
		fmt.Println("func3 defer")
	}()
	fmt.Println("func3")
}

func func1(wg *sync.WaitGroup) {
	defer func() {
		fmt.Println("func1 defer1")
		wg.Done()
	}()
	defer func() {
		fmt.Println("func1 defer2")
	}()
	fmt.Println("func1")
	func2()
	func3()
}

func main() {
	var wg sync.WaitGroup

	wg.Add(1)
	go func1(&wg)
	wg.Wait()
}
