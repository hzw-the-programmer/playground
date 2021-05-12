package main

import (
	"fmt"
	"time"
)

type SafeCounter struct {
	v map[string]int
}

func (c *SafeCounter) Inc(key string) {
	c.v[key]++
}

func (c *SafeCounter) Value(key string) int {
	return c.v[key]
}

func main() {
	c := SafeCounter{v: make(map[string]int)}

	for i := 0; i < 1000; i++ {
		go c.Inc("hzw")
	}

	time.Sleep(time.Second)
	fmt.Println(c.Value("hzw"))
}