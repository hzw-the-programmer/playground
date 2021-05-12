package main

import (
	"log"
)

func f1(i int) {
	defer log.Print("defer1")
	if i == 1 {
		defer log.Print("defer2")
	}
	defer log.Print("defer3")
}

func main() {
	f1(1)
	log.Print()
	f1(2)
}
