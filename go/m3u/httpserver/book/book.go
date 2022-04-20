package book

import (
	"github.com/gofiber/fiber/v2"
	"github.com/jinzhu/gorm"
)

type Book struct {
	gorm.Model
	Title string `json:"title"`
	Author string `json:"author"`
	Rating int `json:"rating"`
}

// curl http://localhost:3000/api/v1/book
func GetBooks(c *fiber.Ctx) error {
	return c.SendString("All books")
}

// curl -X POST http://localhost:3000/api/v1/book
func NewBook(c *fiber.Ctx) error {
	return c.SendString("Adds a new book")
}

// curl http://localhost:3000/api/v1/book/1
func GetBook(c *fiber.Ctx) error {
	return c.SendString("A single book")
}

// curl -X DELETE http://localhost:3000/api/v1/book/1
func DeleteBook(c *fiber.Ctx) error {
	return c.SendString("Deletes a book")
}