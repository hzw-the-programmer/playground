package main

import (
	"context"
	"log"
	"os"
	"time"

	"github.com/go-redis/redis/v8"
)

func main() {
	for _, addr := range os.Args[1:] {
		test(addr)
	}
}

func test(addr string) {
	cli := redis.NewClient(&redis.Options{
		Addr:        addr,
		DialTimeout: 50 * time.Millisecond,
	})
	defer cli.Close()

	ctx := context.Background()

	for i := 0; i < 100; i++ {
		_, err := cli.Ping(ctx).Result()
		if err != nil {
			log.Fatal(err)
		}
	}

	for i := 0; i < 100; i++ {
		_, err := cli.Set(ctx, "hzwcount", i, time.Minute).Result()
		if err != nil {
			log.Fatal(err)
		}

		_, err = cli.Get(ctx, "hzwcount").Result()
		if err != nil {
			log.Fatal(err)
		}

		_, err = cli.Del(ctx, "hzwcount").Result()
		if err != nil {
			log.Fatal(err)
		}
	}
}
