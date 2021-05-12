package main

import (
	"os"
	"log"
	"bufio"
	"fmt"
	"io"
)

func main() {
	file, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	r := bufio.NewReader(file)
	for {
		line, err := r.ReadString('\n')
		if (err == io.EOF) {
			break
		}
		if err != nil {
			log.Fatal(err)
		}
		fmt.Print(line)
	}
}