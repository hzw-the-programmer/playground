package main

import (
	"log"
	"net/http"

	"github.com/GeertJohan/go.rice"
	"github.com/prometheus/client_golang/prometheus/promhttp"
)

func main() {
	http.Handle("/", http.FileServer(rice.MustFindBox("web").HTTPBox()))
	http.Handle("/metrics", promhttp.Handler())
	log.Print(http.ListenAndServe(":1123", nil))
}
