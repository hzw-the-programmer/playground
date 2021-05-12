package handler

import (
	"net/http"
	"time"

	"github.com/gin-gonic/gin"
)

func Sleep(c *gin.Context) {
	time.Sleep(10 * time.Second)
	c.JSON(http.StatusOK, gin.H{
		"hello": "world",
	})
}
