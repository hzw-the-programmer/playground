package main

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

var db = make(map[string]string)

func main() {
	r := gin.Default()
	
	r.GET("/ping", func (c *gin.Context) {
		c.String(http.StatusOK, "pong")
	})

	r.GET("/user/:name", func (c *gin.Context) {
		user := c.Params.ByName("name")
		value, ok := db[user]
		if ok {
			c.JSON(http.StatusOK, gin.H{"user": user, "value": value})
		} else {
			c.JSON(http.StatusOK, gin.H{"user": user, "status": "no value"})
		}
	})

	authorized := r.Group("/", gin.BasicAuth(gin.Accounts{
		"foo": "bar",
		"manu": "123",
	}))

	// curl -d '{"value": "hello"}' -H 'Content-Type: application/json' -u foo:bar localhost:8080/admin
	authorized.POST("admin", func (c *gin.Context) {
		user := c.MustGet(gin.AuthUserKey).(string)

		var json struct {
			Value string `json:"value" binding:"required"`
		}

		if c.Bind(&json) == nil {
			db[user] = json.Value
			c.JSON(http.StatusOK, gin.H{"status": "ok"})
		}
	})

	r.Run(":8080")
}
