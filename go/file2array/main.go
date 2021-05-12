package main

import (
	"fmt"
	"log"
	"os"
)

func main() {
	if len(os.Args) < 2 {
		log.Fatalf("Usage: %s filename", os.Args[0])
	}

	filename := os.Args[1]
	file, err := os.Open(filename)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	b := make([]byte, 1)
	for {
		l, err := file.Read(b)
		if l == 0 {
			log.Print(err)
			break
		}
		fmtStr := "'%c', "
		if b[0] == 0 {
			fmtStr = "%d,\n"
		} else if b[0] == '\'' {
			fmtStr = "'\\%c', "
		}
		fmt.Printf(fmtStr, b[0])
	}
}
