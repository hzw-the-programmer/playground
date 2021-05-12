package main

import (
	"compress/gzip"
	"fmt"
	"io"
	"log"
	"os"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Printf("Usage: %s filename\n", os.Args[0])
	}

	fileName := os.Args[1]
	file, err := os.Open(fileName)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	zr, err := gzip.NewReader(file)
	if err != nil {
		log.Fatal(err)
	}
	defer zr.Close()

	outFileName := fileName + ".out"
	outFile, err := os.Create(outFileName)
	if err != nil {
		log.Fatal(err)
	}
	defer outFile.Close()

	if _, err := io.Copy(outFile, zr); err != nil {
		log.Fatal(err)
	}
}
