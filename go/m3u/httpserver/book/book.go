package book

import (
	"github.com/gofiber/fiber/v2"
	"github.com/jinzhu/gorm"

	"example.com/httpserver/database"
)

type Book struct {
	gorm.Model
	Title string `json:"title"`
	Author string `json:"author"`
	Rating int `json:"rating"`
}

// curl http://localhost:3000/api/v1/book
func GetBooks(c *fiber.Ctx) error {
	var books []Book
	database.DB.Find(&books)
	return c.JSON(books)
}

// curl -X POST http://localhost:3000/api/v1/book
func NewBook(c *fiber.Ctx) error {
	var book Book
	book.Title = "1984"
	book.Author = "George Orwell"
	book.Rating = 5
	database.DB.Create(&book)
	return c.JSON(book)
}

// curl http://localhost:3000/api/v1/book/1
func GetBook(c *fiber.Ctx) error {
	id := c.Params("id")
	var book Book
	database.DB.Find(&book, id)
	return c.JSON(book)
}

// curl -X DELETE http://localhost:3000/api/v1/book/1
func DeleteBook(c *fiber.Ctx) error {
	id := c.Params("id")
	var book Book
	database.DB.First(&book, id)
	if book.Title == "" {
		return c.Status(500).SendString("No book found with given ID")
	}
	database.DB.Delete(&book)
	return c.SendString("Book successfully deleted")
}