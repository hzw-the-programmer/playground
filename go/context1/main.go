package main

import (
	"context"
	"log"
	"sync"
)

func main() {
	var wg sync.WaitGroup
	ctx, cancel := context.WithCancel(context.Background())
	defer func() {
		cancel()
		log.Printf("canceled")
		wg.Wait()
		log.Print("all goroutine finished")
	}()

	for i := 0; i < 10; i++ {
		wg.Add(1)
		go func(i int) {
			select {
			case <-ctx.Done():
				log.Printf("%d error: %s", i, ctx.Err())
				break
			}
			wg.Done()
		}(i)
	}
	log.Print("all goroutine started")
}

/*
2020/10/12 21:56:40 all goroutine started
2020/10/12 21:56:40 4 error: context canceled
2020/10/12 21:56:40 9 error: context canceled
2020/10/12 21:56:40 8 error: context canceled
2020/10/12 21:56:40 7 error: context canceled
2020/10/12 21:56:40 5 error: context canceled
2020/10/12 21:56:40 2 error: context canceled
2020/10/12 21:56:40 6 error: context canceled
2020/10/12 21:56:40 1 error: context canceled
2020/10/12 21:56:40 0 error: context canceled
2020/10/12 21:56:40 3 error: context canceled
2020/10/12 21:56:40 canceled
2020/10/12 21:56:40 all goroutine finished
*/

/*
2020/10/12 21:57:17 all goroutine started
2020/10/12 21:57:17 canceled
2020/10/12 21:57:17 6 error: context canceled
2020/10/12 21:57:17 9 error: context canceled
2020/10/12 21:57:17 8 error: context canceled
2020/10/12 21:57:17 7 error: context canceled
2020/10/12 21:57:17 5 error: context canceled
2020/10/12 21:57:17 3 error: context canceled
2020/10/12 21:57:17 1 error: context canceled
2020/10/12 21:57:17 4 error: context canceled
2020/10/12 21:57:17 0 error: context canceled
2020/10/12 21:57:17 2 error: context canceled
2020/10/12 21:57:17 all goroutine finished
*/