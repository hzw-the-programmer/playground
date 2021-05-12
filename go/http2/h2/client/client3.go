package client

import (
	"context"
	"io/ioutil"
	"log"
	"net/http"
	"time"
)

func Run3(addr string) {
	// ctx, cancel := context.WithTimeout(context.Background(), 11000*time.Millisecond)
	// ctx, cancel := context.WithTimeout(context.Background(), 500*time.Millisecond)
	ctx, cancel := context.Background(), func() {}
	defer cancel()

	req, err := http.NewRequestWithContext(ctx, http.MethodGet, "http://localhost:8080/sleep", nil)
	if err != nil {
		log.Fatal(err)
	}

	start := time.Now()
	res, err := http.DefaultClient.Do(req)
	log.Print(time.Now().Sub(start))
	if err != nil {
		log.Fatal(err)
	}
	defer res.Body.Close()

	body, err := ioutil.ReadAll(res.Body)
	if err != nil {
		log.Fatal(err)
	}

	log.Print(string(body))
}
