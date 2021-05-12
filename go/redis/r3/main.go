package main

import (
	"bytes"
	"context"
	"encoding/gob"
	"log"
	"os"

	"github.com/go-redis/redis/v8"
)

func main() {
	addr := os.Args[1]

	o := redis.Options{Addr: addr}
	c := redis.NewClient(&o)

	ctx := context.Background()

	log.Print(c.Ping(ctx))
	// log.Print(c.Ping())
	sscmd := c.Keys(ctx, "session_*")
	// sscmd := c.Keys("session_*")
	log.Print(sscmd)
	ss, err := sscmd.Result()
	if err != nil {
		log.Fatal(err)
	}
	for _, s := range ss {
		log.Print(s)
		sc := c.Get(ctx, s)
		// sc := c.Get(s)
		// log.Print(sc)
		b, err := sc.Bytes()
		if err != nil {
			log.Print(err)
		}
		var v = map[interface{}]interface{}{}
		dec := gob.NewDecoder(bytes.NewBuffer(b))
		err = dec.Decode(&v)
		if err == nil {
			log.Print(v)
			continue
		}
		log.Print(sc.Val())
	}
}
