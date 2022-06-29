package main

import (
	"bytes"
	"compress/gzip"
	"fmt"
	"io"
	"log"
	"os"
	"time"
)

func main() {
	var buf bytes.Buffer
	zw := gzip.NewWriter(&buf)

	zw.Name = "a-new-hope.txt"
	zw.Comment = "an epic space opera by Gorge Lucas"
	zw.ModTime = time.Date(1987, time.May, 25, 0, 0, 0, 0, time.UTC)

	_, err := zw.Write([]byte("A long time ago in a galaxy far, far away..."))
	if err != nil {
		log.Fatal(err)
	}

	if err := zw.Close(); err != nil {
		log.Fatal(err)
	}

	// fmt.Printf("%x\n", buf.Bytes());
	// fmt.Printf("%v\n", buf.Bytes());
	// fmt.Printf("%+v\n", buf.Bytes());

	for i, b := range buf.Bytes() {
		fmt.Printf("0x%02x, ", b)
		if i % 10 == 9 {
			fmt.Println()
		}
	}

	fmt.Println()

	/*
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
	*/

	zr, err := gzip.NewReader(&buf)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Name: %s\nComment: %s\nModTime: %s\n\n", zr.Name, zr.Comment, zr.ModTime.UTC())
	if _, err := io.Copy(os.Stdout, zr); err != nil {
		log.Fatal(err)
	}
	if err := zr.Close(); err != nil {
		log.Fatal(err)
	}
}
