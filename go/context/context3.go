package main

import (
	"context"
	"log"
	"sync"
	"time"
)

func work(ctx context.Context, out chan<- time.Time) {
	for {
		select {
		case t := <-time.Tick(1 * time.Second):
			out <- t
		case <-ctx.Done():
			log.Printf("work done: %s", ctx.Err())
			close(out)
			return
		}
	}
}

func main() {
	var wg sync.WaitGroup

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()
	out := make(chan time.Time)

	wg.Add(1)
	go func() {
		defer wg.Done()
		work(ctx, out)
	}()

	wg.Add(1)
	go func() {
		defer wg.Done()
		for t := range out {
			log.Printf("out: %s", t)
		}
	}()
	
	wg.Wait()
}
