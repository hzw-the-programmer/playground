package main

import (
	"bufio"
	"context"
	"log"
	"os"
	"time"
)

func main() {
	ctx, cf := context.WithCancel(context.Background())
	go func() {
		i := 0
		j := 0
		for {
			select {
			case <-time.Tick(3 * time.Second):
				i++
				log.Print(i)
			case <-ctx.Done():
				log.Print(ctx.Err())
				j++
				if j == 2 {
					return
				}
			}
		}
	}()
	time.AfterFunc(10 * time.Second, func() {
		cf()
	})
	bufio.NewReader(os.Stdin).ReadLine()
}
