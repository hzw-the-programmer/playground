package main

import (
	"fmt"
	"context"
	"time"
)

func main() {
	ctx, cancel := context.WithCancel(context.Background())

	go func() {
		select {
		case <-ctx.Done():
			fmt.Println(ctx.Err())
			return
		}
	}()

	cancel()
	time.Sleep(1 * time.Second)
}