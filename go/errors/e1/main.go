package main

import (
	"errors"
	"log"
)

func main() {
	e1 := errors.New("Error")
	e2 := errors.New("Error")
	log.Println(e1 == e2)
}
