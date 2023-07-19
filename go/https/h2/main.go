package main

import (
	"log"
	"net/http"
)

func hello(w http.ResponseWriter, req *http.Request) {
	w.Write([]byte("hello world!"))
}

// openssl genrsa -out server.key 2048
// openssl req -new -x509 -sha256 -key server.key -out server.crt -days 3650
func main() {
	http.HandleFunc("/", hello)
	err := http.ListenAndServeTLS(":4433", "server.crt", "server.key", nil)
	if err != nil {
		log.Fatal(err)
	}
}
