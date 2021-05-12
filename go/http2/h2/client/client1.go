package client

import (
	"fmt"
	"log"
	"net/http"
)

func Run1(addr string) {
	method := http.MethodGet
	url := fmt.Sprintf("http://%s/two", addr)

	req, err := http.NewRequest(method, url, nil)
	if err != nil {
		panic(err)
	}

	res, err := http.DefaultClient.Do(req)
	if err != nil {
		panic(err)
	}
	defer res.Body.Close()

	for k, v := range res.Header {
		for _, vv := range v {
			log.Println(k, vv)
		}
	}
}
