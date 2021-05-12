package handler

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

type Book struct {
	Name   string
	Author string
}

func QueryBooks(c *gin.Context) {
	books := []*Book{
		&Book{
			Name:   "数学之美",
			Author: "吴军",
		},
		&Book{
			Name:   "产品方法论",
			Author: "俞军",
		},
	}

	var result []*Book

	book := &Book{}
	if err := c.ShouldBind(book); err == nil && (book.Name != "" || book.Author != "") {
		for _, b := range books {
			if b.Name == book.Name || b.Author == book.Author {
				result = append(result, b)
			}
		}
	} else {
		result = books
	}

	c.JSON(http.StatusOK, result)
}
