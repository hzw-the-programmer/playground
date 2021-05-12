package main

import (
	"fmt"
	"context"
)

type Ctx struct {
	context.Context
}

/*
func (c *Ctx) Done() (done <-chan struct{}) {
	return c.Context.Done()
	//return
}
*/

func main() {
	ctx1, cancel := context.WithCancel(context.Background())
	ctx2 := Ctx{ctx1}
	fmt.Println(ctx1.Done(), ctx2.Done())
	cancel()
	select {
	case <-ctx2.Done():
		fmt.Printf("ctx2 is closed: %s.\n", ctx2.Err())
	default:
		fmt.Println("ctx2 is open.")
	}
}
