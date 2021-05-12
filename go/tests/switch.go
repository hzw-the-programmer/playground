package main

import (
	"log"
)

func main() {
	log.Print("before for")
	for i := 0; i < 4; i++ {
		log.Printf("before %d", i)
		switch i {
		case 0, 1:
			if i == 0 {
				break
			}
			log.Print("hehe")
		case 2:
			continue
		}
		log.Printf("after %d", i)
	}
	log.Print("after for")
}
