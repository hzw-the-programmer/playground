package api

import (
	"net/http"
	"log"
)

type appError struct {
	Error error
	Message string
	Code int
}

type appHandler func(http.ResponseWriter, *http.Request) *appError

func (fn appHandler) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	if err := fn(w, r); err != nil {
		log.Printf("%v", err.Error)
		http.Error(w, err.Message, err.Code)
	}
}