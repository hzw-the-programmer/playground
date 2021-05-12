package main

import (
	"log"
)

func main() {
	log.Print("enter main")
	/*
	for i := 0; i < 3; i++ {
		log.Print("enter for", i)
		defer func(i int) {
			log.Print("defer", i)
		}(i)
		log.Print("leave for", i)
	}
	*/
	{
		log.Print("enter block")
		defer func() {
			log.Print("defer")
		}()
		log.Print("leave block")
	}
	log.Print("leave main")
}
