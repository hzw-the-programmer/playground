/*
Copyright © 2022 NAME HERE <EMAIL ADDRESS>

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/
package cmd

import (
	"fmt"
	"log"

	"github.com/spf13/cobra"
	"github.com/gofiber/fiber/v2"
	"gorm.io/gorm"
	"gorm.io/driver/mysql"

	"example.com/httpserver/book"
	"example.com/httpserver/database"
)

var _ = fmt.Printf

// fiberCmd represents the fiber command
var fiberCmd = &cobra.Command{
	Use:   "fiber",
	Short: "A brief description of your command",
	Long: `A longer description that spans multiple lines and likely contains examples
and usage of using your command. For example:

Cobra is a CLI library for Go that empowers applications.
This application is a tool to generate the needed files
to quickly create a Cobra application.`,
	Run: func(cmd *cobra.Command, args []string) {
		dsn := "gorm1:gorM@123@tcp(127.0.0.1:3306)/gorm1?charset=utf8mb4&parseTime=True&loc=Local"
		fiberCmdRun(dsn)
	},
}

func init() {
	rootCmd.AddCommand(fiberCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// fiberCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// fiberCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}

func fiberCmdRun(dsn string) {
	app := fiber.New()

	initDatabase(dsn)
	setupRoutes(app)

	log.Fatal(app.Listen(":3000"))
}

func helloWorld(c *fiber.Ctx) error {
	return c.SendString("Hello World!")
}

func setupRoutes(app *fiber.App) {
	app.Get("/api/v1/book", book.GetBooks)
	app.Post("/api/v1/book", book.NewBook)
	app.Get("/api/v1/book/:id", book.GetBook)
	app.Delete("/api/v1/book/:id", book.DeleteBook)
}

func initDatabase(dsn string) {
	var err error
	database.DB, err = gorm.Open(mysql.Open(dsn), &gorm.Config{})
	if err != nil {
		log.Fatal(err)
	}
	log.Println("Database connection successfully opened")

	database.DB.AutoMigrate(&book.Book{})
	log.Println("Database migrated")
}