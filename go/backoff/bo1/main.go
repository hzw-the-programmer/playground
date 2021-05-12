package main

import (
	"fmt"
	"log"
	"time"

	"github.com/cenkalti/backoff/v4"
)

func main() {
	bo := backoff.NewConstantBackOff(500 * time.Millisecond)
	err := backoff.Retry(func() func() error {
		count := 0
		return func() error {
			log.Print("count: ", count)
			if count == 5 {
				return nil
			}
			err := fmt.Errorf("error: %d", count)
			count++
			return err
		}
	}(), bo)
	log.Print("return: ", err)
}
