package main

import (
	"fmt"
	"log"
	"os"

	"golang.org/x/text/encoding/unicode"
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

	decoder := unicode.UTF16(unicode.LittleEndian, unicode.IgnoreBOM).NewDecoder()
	r := decoder.Reader(file)
	r = file

	b := make([]byte, 1)
	i := 0
	j := 0
	for {
		len, err := r.Read(b)
		
		if err != nil {
			log.Print(err)
			break
		}
		
		if len == 0 {
			log.Print("finish")
			break
		}
		
		//fmt.Printf("%02x", b[0])
		j++
		
		if b[0] == 0 {
			fmt.Printf("%d: %d\n", i, j)
			i++
			j = 0
		}
	}
	log.Printf("%d", i)
}
