package main

import (
	"io"
	"log"
	"os"

	"golang.org/x/text/encoding/unicode"
)

func main() {
	if len(os.Args) < 2 {
		log.Fatalf("Usage: %s filename", os.Args[0])
	}

	filename := os.Args[1]

	rFile, err := os.Open(filename)
	if err != nil {
		log.Fatal(err)
	}
	defer rFile.Close()

	decoder := unicode.UTF16(unicode.LittleEndian, unicode.IgnoreBOM).NewDecoder()
	r := decoder.Reader(rFile)

	wFile, err := os.Create(filename + "_out")
	if err != nil {
		log.Fatal(err)
	}
	defer wFile.Close()

	written, err := io.Copy(wFile, r)
	if err != nil {
		log.Fatal(err)
	}
	log.Printf("%d", written)
}
