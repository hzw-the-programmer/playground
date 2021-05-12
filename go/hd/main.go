package main

import (
	"fmt"
	"io"
	"log"
	"os"
)

func main() {
	if len(os.Args) != 2 {
		log.Fatalf("Usage: %s path", os.Args[0])
	}

	file, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var bytes [10]byte
	for {
		n, err := file.Read(bytes[:])
		if err != nil {
			if err != io.EOF {
				log.Fatal(err)
			} else {
				break
			}
		}
		for _, b := range bytes[:n] {
			fmt.Printf("0x%02x, ", b)
		}
		fmt.Print("\n")
	}
}
