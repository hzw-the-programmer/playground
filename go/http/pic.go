package main

import (
	"io/ioutil"
	"log"
	"net/http"
	"net/url"
	"os"
	"path"
)

func main() {
	urlr := os.Args[1]
	referer := os.Args[2]
	urlp, err := url.Parse(urlr)
	if err != nil {
		log.Fatal(err)
	}
	log.Print(urlp.Path)
	log.Print(path.Dir(urlp.Path))
	log.Print(path.Base(urlp.Path))
	dir := "." + path.Dir(urlp.Path)
	fn := path.Base(urlp.Path)
	err = os.MkdirAll(dir, 0775)
	if err != nil {
		log.Fatal(err)
	}

	req, err := http.NewRequest("GET", urlr, nil)
	if err != nil {
		log.Fatal(err)
	}
	req.Header.Set("Referer", referer)
	resp, err := http.DefaultClient.Do(req)
	defer resp.Body.Close()
	b, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		log.Fatal(err)
	}
	err = ioutil.WriteFile(dir + "/" + fn, b, 0666)
	if err != nil {
		log.Fatal(err)
	}
}
