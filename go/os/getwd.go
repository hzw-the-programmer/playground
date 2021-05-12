package main

import (
	"log"
	"os"
)

func main() {
	log.SetFlags(log.LstdFlags | log.Lshortfile)
	dir , err := os.Getwd()
	if err != nil {
		log.Fatal(err)
	}
	log.Print(dir)
}
