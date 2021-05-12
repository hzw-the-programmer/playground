package main

//GOOS=windows GOARCH=amd64 go build filepath.go

import (
	"log"
	"path/filepath"
)

func main() {
	c := 'c'
	log.Printf("%T", c)
	log.Printf("%T", filepath.Separator)
	log.Print(filepath.Separator)
	log.Print(string(filepath.Separator))
}
