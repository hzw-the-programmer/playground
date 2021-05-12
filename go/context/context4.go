package main

import (
	"bufio"
	"context"
	"log"
	"os"
	"time"
)

func main() {
	ctx, cancel := context.WithTimeout(context.Background(), 3*time.Second)
	go func() {
		timer := time.NewTicker(300 * time.Millisecond)
		defer timer.Stop()
		for {
			select {
			case <-ctx.Done():
				log.Print(ctx.Err())
				go func() {
					for {
						select {
						case _, ok := <-timer.C:
							log.Println("h tick", ok)
						}
					}
				}()
				return
			case <-timer.C:
				log.Print("tick")
			}
		}
	}()

	r := bufio.NewReader(os.Stdin)
	r.ReadLine()
	cancel()
	time.Sleep(1 * time.Second)
}
