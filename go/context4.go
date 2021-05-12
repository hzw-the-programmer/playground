package main

import (
	"fmt"
	"time"
	"context"
)

func main() {
	ctx, cancel := context.WithTimeout(context.Background(), 999 * time.Millisecond)
	defer cancel()

	select {
	case <-ctx.Done():
		fmt.Println(ctx.Err())
	case <-time.After(1 * time.Second):
		fmt.Println("overslept")
	}
}