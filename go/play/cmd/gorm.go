/*
Copyright © 2022 NAME HERE <EMAIL ADDRESS>

*/
package cmd

import (
	//"fmt"
	"log"

	"github.com/spf13/cobra"
	"gorm.io/gorm"
	"gorm.io/driver/mysql"
)

// gormCmd represents the gorm command
var gormCmd = &cobra.Command{
	Use:   "gorm",
	Short: "A brief description of your command",
	Long: `A longer description that spans multiple lines and likely contains examples
and usage of using your command. For example:

Cobra is a CLI library for Go that empowers applications.
This application is a tool to generate the needed files
to quickly create a Cobra application.`,
	Run: func(cmd *cobra.Command, args []string) {
		if err := gormCmdRun(); err != nil {
			log.Fatal(err)
		}
	},
}

func init() {
	rootCmd.AddCommand(gormCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// gormCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// gormCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}

type Product struct {
	gorm.Model
	Code string
	Price uint
}

func gormCmdRun() error {
	dsn := "root:RooT@123@tcp(127.0.0.1:3306)/gorm?charset=utf8mb4&parseTime=True&loc=Local"
	
	db, err := gorm.Open(mysql.Open(dsn), &gorm.Config{})
	if err != nil {
		return err
	}

	db.AutoMigrate(&Product{})

	db.Create(&Product{Code: "D42", Price: 100})

	var product Product
	//db.First(&product, 1)
	//log.Print(product)
	db.First(&product, "code = ?", "D42")
	log.Print(product)

	db.Model(&product).Update("Price", 200)
	log.Print(product)

	db.Model(&product).Updates(Product{Price: 300, Code: "F42"})
	log.Print(product)

	db.Model(&product).Updates(map[string]interface{}{"Price": 400, "Code": "F43"})
	log.Print(product)

	db.Delete(&product, 1)

	return nil
}