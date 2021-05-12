package main

import (
	"io"
	"log"
	"sync"
)

func main() {
	bs := [][]byte{[]byte("123"), []byte("456"), []byte("789")}
	var wg sync.WaitGroup
	r, w := io.Pipe()
	for i := 0; i < len(bs); i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			w.Write(bs[i])
		}(i)
	}
	bs1 := [1]byte{}
	for i := 0; i < 9; i++ {
		r.Read(bs1[:1])
		log.Print(string(bs1[:1]))
	}
	wg.Wait()
}
