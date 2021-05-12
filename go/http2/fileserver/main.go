package main

import (
	"fmt"
	"log"
	"net/http"
	"os"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Printf("Usage: %s webroot\n", os.Args[0])
		return
	}
	webroot := os.Args[1]
	log.Fatal(http.ListenAndServe(":8080", http.FileServer(http.Dir(webroot))))
}
