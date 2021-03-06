package main

import (
	"time"
	"context"
	"fmt"
)

func main() {
	d := time.Now().Add(999 * time.Millisecond)
	ctx, cancel := context.WithDeadline(context.Background(), d)
	defer cancel()

	select {
	case <-ctx.Done():
		fmt.Println(ctx.Err())
	case <-time.After(1 * time.Second):
		fmt.Println("overslept")
	}
}