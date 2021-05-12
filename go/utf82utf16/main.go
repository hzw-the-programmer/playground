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

	wFile, err := os.Create(filename + "_out")
	if err != nil {
		log.Fatal(err)
	}
	defer wFile.Close()

	encoder := unicode.UTF16(unicode.LittleEndian, unicode.IgnoreBOM).NewEncoder()
	w := encoder.Writer(wFile)

	written, err := io.Copy(w, rFile)
	if err != nil {
		log.Fatal(err)
	}
	log.Printf("%d", written)
}
