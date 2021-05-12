package main

import (
	"fmt"
	"context"
)

func main() {
	ctx, _ := context.WithCancel(context.Background())
	fmt.Println(ctx)
}