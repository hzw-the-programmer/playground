package main

import (
	"fmt"
	"time"
)

func debounce(d time.Duration, f func(a int)) func(a int) {
	t := time.NewTimer(d)
	t.Stop()
	var arg int
	go func() {
		for {
			select {
			case <-t.C:
				f(arg)
			}
		}
	}()

	return func(a int) {
		arg = a
		t.Reset(d)
	}
}

func main() {
	f := debounce(300, func(a int) {
		fmt.Println(a)
	})

	for i := 0; i < 10; i++ {
		f(i)
	}

	time.Sleep(1 * time.Second)
}
