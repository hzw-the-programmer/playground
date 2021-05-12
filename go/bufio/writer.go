package main

import (
	"bufio"
	"bytes"
	"log"
)

func main() {
	var buf bytes.Buffer
	writer := bufio.NewWriterSize(&buf, 5)
	writer.Write([]byte("hzw"))
	log.Print(&buf)
	writer.Write([]byte("wz"))
	log.Print(&buf)
	// writer.Write([]byte("12"))
	// writer.Write([]byte("12345"))
	writer.Write([]byte("123456"))
	log.Print(&buf)
}
