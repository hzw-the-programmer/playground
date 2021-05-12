package main

import (
	"log"
	"net/http"
)

func main() {
	err := http.ListenAndServe(":9898", nil)
	log.Print(err)
}
