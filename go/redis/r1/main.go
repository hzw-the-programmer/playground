package main

import (
	"context"
	"log"
	"os"

	"github.com/go-redis/redis/v8"
)

func main() {
	addr := os.Args[1]
	ctx := context.Background()
	rdb := redis.NewClient(&redis.Options{
		Addr: addr,
	})

	err := rdb.Set(ctx, "k1", "v1", 0).Err()
	if err != nil {
		log.Fatal(err)
	}

	val, err := rdb.Get(ctx, "k1").Result()
	if err != nil {
		log.Fatal(err)
	}

	log.Print("k1", val)
}
