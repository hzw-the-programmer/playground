package main

import (
	"log"
	"net/http"

	"github.com/gin-gonic/gin"
)

func main() {
	e := gin.New()
	e.Use(gin.Logger())
	e.Use(gin.Recovery())

	e.GET("/", func(c *gin.Context) {
		c.String(http.StatusOK, "hello world")
	})

	e.GET("/panic", func(c *gin.Context) {
		var m map[string]string
		m["key"] = "value"
	})

	log.Print(e.Run())
}
