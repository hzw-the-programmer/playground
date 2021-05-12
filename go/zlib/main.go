package main

import (
	"bytes"
	"compress/zlib"
	"fmt"
	"io"
	"log"
)

func main() {
	var buf bytes.Buffer

	w := zlib.NewWriter(&buf)
	w.Write([]byte("hello world\n"))
	w.Close()

	b := make([]byte, 1)
	i := 0
	for {
		_, err := buf.Read(b)
		if err != nil {
			if err == io.EOF {
				fmt.Print("\n")
				break
			} else {
				log.Fatal(err)
			}
		}
		fmt.Printf("0x%02x,", b[0])
		if i%10 == 9 {
			fmt.Print("\n")
		} else {
			fmt.Print(" ")
		}
		i++
	}
}
