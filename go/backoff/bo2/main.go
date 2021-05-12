package main

import (
	"context"
	"fmt"
	"log"
	"time"

	"github.com/cenkalti/backoff/v4"
)

func main() {
	test1()
	test2()
}

func test1() {
	bo := backoff.NewConstantBackOff(500 * time.Millisecond)
	ctx, cancel := context.WithTimeout(context.Background(), 2300*time.Millisecond)
	defer cancel()
	bo1 := backoff.WithContext(bo, ctx)
	err := backoff.Retry(func() func() error {
		count := 0
		return func() error {
			log.Print("count: ", count)
			err := fmt.Errorf("error: %d", count)
			count++
			return err
		}
	}(), bo1)
	log.Print("return: ", err)
	s := time.Now()
	<-ctx.Done()
	log.Print(time.Now().Sub(s))
}

func test2() {
	bo := backoff.NewConstantBackOff(500 * time.Millisecond)
	ctx, cancel := context.WithTimeout(context.Background(), 2300*time.Millisecond)
	defer cancel()
	bo1 := backoff.WithContext(bo, ctx)

	go func() {
		time.Sleep(1300 * time.Millisecond)
		cancel()
	}()

	err := backoff.Retry(func() func() error {
		count := 0
		return func() error {
			log.Print("count: ", count)
			err := fmt.Errorf("error: %d", count)
			count++
			return err
		}
	}(), bo1)
	log.Print("return: ", err)
}
