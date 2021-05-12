package main

import (
	"bytes"
	"context"
	"encoding/gob"
	"log"
	"os"
	"time"

	"github.com/gomodule/redigo/redis"
)

func main() {
	start := time.Now()
	defer func() {
		log.Print(time.Now().Sub(start))
	}()

	addr := os.Args[1]

	ctx, cancel := context.WithTimeout(context.Background(), 3*time.Second)
	defer cancel()

	c, err := redis.DialContext(ctx, "tcp", addr)
	if err != nil {
		log.Print(err)
		return
	}
	defer c.Close()

	sks, err := redis.Strings(c.Do("KEYS", "session_*"))
	if err != nil {
		log.Fatal(err)
	}

	for _, sk := range sks {
		log.Print(sk)
		sv, err := redis.Bytes(c.Do("GET", sk))
		if err != nil {
			log.Fatal(err)
		}
		var v map[interface{}]interface{}
		dec := gob.NewDecoder(bytes.NewBuffer(sv))
		err = dec.Decode(&v)
		if err == nil {
			log.Print(v)
			continue
		}
		log.Print(string(sv))
	}
}
