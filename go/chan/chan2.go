package main

import (
	"fmt"
	"log"
	"sync"
	"time"
)

type mychan struct {
	c      chan string
	rwmu   sync.RWMutex
	closed bool
}

func (c *mychan) send(msg string) bool {
	// c.rwmu.RLock()
	// closed := c.closed
	// c.rwmu.RUnlock()
	// time.Sleep(4 * time.Second)
	// if closed {
	// 	return false
	// }
	// c.c <- msg
	// return true
	
	c.rwmu.RLock()
	defer c.rwmu.RUnlock()
	time.Sleep(4 * time.Second)
	if c.closed {
		return false
	}
	c.c <- msg
	return true
}

func (c *mychan) close() {
	// c.rwmu.RLock()
	// closed := c.closed
	// time.Sleep(1 * time.Second)
	// c.rwmu.RUnlock()
	// if closed {
	// 	return
	// }
	// close(c.c)
	// c.closed = true

	// c.rwmu.Lock()
	// defer c.rwmu.Unlock()
	// if c.closed {
	// 	return
	// }
	// close(c.c)
	// c.closed = true

	c.rwmu.Lock()
	if c.closed {
		c.rwmu.Unlock()
		return
	}
	c.closed = true
	c.rwmu.Unlock()
	
	close(c.c)
}

func main() {
	var wg sync.WaitGroup

	c := mychan{
		c: make(chan string),
	}

	for i := 0; i < 1000; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			j := 0
			for {
				if !c.send(fmt.Sprintf("%d: %d", i, j)) {
					break
				}
				j++
				time.Sleep(1 * time.Second)
			}
		}(i)
	}

	wg.Add(1)
	go func() {
		defer wg.Done()
		for msg := range c.c {
			log.Print(msg)
		}
	}()

	for i := 0; i < 1000; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			time.Sleep(3 * time.Second)
			c.close()
		}()
	}

	wg.Wait()
}
