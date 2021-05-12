package main

import (
	"log"
	"net/http"
)

func hawkeye(w http.ResponseWriter, r *http.Request) {
	log.Print("ok")
}

func main() {
	http.HandleFunc("/hawkeye", hawkeye)
	server := &http.Server{Addr: ":1123"}
	err := server.ListenAndServe()
	if err != nil {
		log.Fatal(err)
	}
}
