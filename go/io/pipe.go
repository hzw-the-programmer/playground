package main

import (
	"io"
	"log"
	"sync"
)

func main() {
	r, w := io.Pipe()
	var wg sync.WaitGroup
	for i := 0; i < 26; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			b := [1]byte{}
			r.Read(b[:1])
			log.Print(string(b[:1]))
		}()
	}
	b := []byte("abcdefghijklmnopqrstuvwxyz")
	w.Write(b)
	wg.Wait()
}
