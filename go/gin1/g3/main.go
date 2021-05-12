package main

import (
	"crypto/rand"
	"log"
	"net/http"
	"os"

	"github.com/boj/redistore"
	"github.com/gin-contrib/sessions"
	"github.com/gin-contrib/sessions/redis"
	"github.com/gin-gonic/gin"
)

func main() {
	addr := os.Args[1]

	e := gin.Default()

	authKey := make([]byte, 32)
	encKey := make([]byte, 32)
	if n, err := rand.Read(authKey); err != nil || n != 32 {
		log.Fatal(err, n)
	}
	log.Print(authKey)
	log.Printf("%v", authKey)
	log.Printf("%#v", authKey)
	if n, err := rand.Read(encKey); err != nil || n != 32 {
		log.Fatal(err, n)
	}
	log.Print(encKey)
	log.Printf("%v", encKey)
	log.Printf("%#v", encKey)
	store, err := redis.NewStore(3, "tcp", addr, "", authKey, encKey)
	if err != nil {
		log.Fatal(err)
	}
	err, redisStore := redis.GetRedisStore(store)
	if err != nil {
		log.Fatal(err)
	}
	redisStore.SetSerializer(redistore.JSONSerializer{})
	e.Use(sessions.Sessions("g3", store))

	e.GET("/", func(c *gin.Context) {
		s := sessions.Default(c)
		count, ok := s.Get("count").(float64)
		if !ok {
			log.Print("!ok")
			count = 0
		}
		s.Set("count", count+1)
		s.Save()
		c.String(http.StatusOK, "count: %.0f", count)
	})

	if err := e.Run(); err != nil {
		log.Fatal(err)
	}
}
