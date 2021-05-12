package main

import (
	"fmt"
	"runtime"
)

func func1() {
	fmt.Println("func1")
	_, file, line, ok := runtime.Caller(0)
	fmt.Println(file, line, ok)
	_, file, line, ok = runtime.Caller(0)
	fmt.Println(file, line, ok)
	_, file, line, ok = runtime.Caller(1)
	fmt.Println(file, line, ok)
	_, file, line, ok = runtime.Caller(1)
	fmt.Println(file, line, ok)
	_, file, line, ok = runtime.Caller(2)
	fmt.Println(file, line, ok)
}

func main() {
	func1()
}
