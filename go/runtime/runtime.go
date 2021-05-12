package main

import (
	"log"
	"runtime"
)

func func1(d int) {
	_, file, line, ok := runtime.Caller(d)
	log.Printf("%s %d %t", file, line, ok)
}

func func2() {
	// func1(2)
	func1(1)
}

func main() {
	_, file, line, ok := runtime.Caller(0)
	log.Printf("%s %d %t", file, line, ok)

	func1(1)
	func2()
	func1(-1)
}
