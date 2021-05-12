package main

import (
	"log"
	"sync"
	"time"
)

func gen(nums ...int) <-chan int {
	out := make(chan int, len(nums))
	for _, n := range nums {
		out <- n
	}
	close(out)
	return out
}

func sq(in <-chan int) <-chan int {
	out := make(chan int)
	go func() {
		for n := range in {
			out <-n
		}
		close(out)
	}()
	return out
}

func merge(done <-chan struct{}, cs ...<-chan int) <-chan int {
	out := make(chan int)

	var wg sync.WaitGroup
	output := func(c <-chan int, i int) {
		log.Printf("enter %d.\n", i)
		for n := range c {
			log.Printf("%d before write.\n", i)
			select {
			case out <- n:
			case <-done:
				log.Printf("%d done.\n", i)
			}
			log.Printf("%d after write.\n", i)
		}
		wg.Done()
		log.Printf("leave %d.\n", i)
	}
	wg.Add(len(cs))
	for i, c := range cs {
		go output(c, i)
	}
	go func() {
		wg.Wait()
		close(out)
	}()

	return out
}

func main() {
	in := gen(1, 2)
	//in := gen(1, 2, 3)

	// fanout
	c1 := sq(in)
	c2 := sq(in)

	done := make(chan struct{})

	out := merge(done, c1, c2)

	log.Println("before receive.")
	log.Println(<-out)
	log.Println("after receive.")

	done <- struct{}{}
	//close(done)

	/*
	log.Println("before receive.")
	log.Println(<-out)
	log.Println("after receive.")
	*/

	time.Sleep(time.Millisecond)
}