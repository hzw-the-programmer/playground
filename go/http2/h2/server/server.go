package server

import (
	// "github.com/gin-contrib/sessions"
	// "github.com/gin-contrib/sessions/redis"
	"github.com/gin-gonic/gin"

	"h2/handler"
)

func Run(addr string) {
	r := gin.Default()

	// store, err := redis.NewStore(3, "tcp", "localhost:6379", "", []byte("12345678"))
	// if err != nil {
	// 	panic(err)
	// }
	// r.Use(sessions.SessionsMany([]string{"hzw", "hzw1", "hzw2"}, store))
	r.GET("/two", handler.Two)
	r.GET("/one", handler.One)
	r.GET("/st1", handler.St1)
	r.GET("/st2", handler.St2)
	r.GET("/step1", handler.Step1)
	r.GET("/step2", handler.Step2)

	r.GET("/books", handler.QueryBooks)
	r.POST("/books", handler.QueryBooks)

	r.GET("/sleep", handler.Sleep)

	r.Run(addr)
}
