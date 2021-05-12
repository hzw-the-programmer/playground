package main

import (
	"log"
	"net/http"
	"os"
)

func main() {
	if len(os.Args) < 2 {
		log.Fatalf("Usage: %s file", os.Args[0])
	}
	file := os.Args[1]
	http.HandleFunc("/arabic", func(w http.ResponseWriter, r *http.Request) {
		http.ServeFile(w, r, file)
	})
	err := http.ListenAndServe(":8080", nil)
	if err != nil {
		log.Fatal(err)
	}
}
