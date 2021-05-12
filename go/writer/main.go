package main

import (
	"os"
)

func main() {
	tw := ArrayWriter{os.Stdout, false, 0}
	tw.Write([]byte("hello"))
	tw.Write([]byte{0})
	tw.Write([]byte("world"))
	tw.Write([]byte{0})

	tw = ArrayWriter{os.Stdout, true, 0}
	tw.Write([]byte{0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4})
}
