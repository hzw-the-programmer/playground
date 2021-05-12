package main

import (
	"bufio"
	"context"
	"log"
	"os"
	"time"
)

func main() {
	ctx, cf := context.WithDeadline(context.Background(), time.Now().Add(10*time.Second))
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
					time.Sleep(4 * time.Second)
				} else if j == 3 {
					return
				}
			}
		}
	}()
	time.Sleep(13 * time.Second)
	cf()
	log.Print("cf called")
	bufio.NewReader(os.Stdin).ReadLine()
}
