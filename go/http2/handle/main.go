package main

import (
	"fmt"
	"log"
	"net/http"
	"sync"
)

type countHandle struct {
	mu sync.Mutex
	n  int
}

func (h *countHandle) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	h.mu.Lock()
	defer h.mu.Unlock()
	h.n++
	fmt.Fprintf(w, "count is %d\n", h.n)
}

func main() {
	http.Handle("/count", new(countHandle))
	log.Fatal(http.ListenAndServe(":8080", nil))
}
