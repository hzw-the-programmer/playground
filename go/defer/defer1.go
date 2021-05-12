package main

import (
	"log"
)

func f1(i int) {
	if i%2 == 0 {
		return
	}
	defer func() {
		log.Println("defer executed", i)
	}()
}

func main() {
	f1(0)
	f1(1)
}
