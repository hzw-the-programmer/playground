package client

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"

	"github.com/gin-gonic/gin/binding"

	"h2/handler"
)

func Run(addr string) {
	book := &handler.Book{
		Name: "数学之美",
	}
	bookJson, err := json.Marshal(book)
	if err != nil {
		panic(err)
	}

	req, err := http.NewRequest(http.MethodPost, "http://"+addr+"/books", bytes.NewBuffer(bookJson))
	if err != nil {
		panic(err)
	}
	req.Header.Add("Content-Type", binding.MIMEJSON)

	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		return
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		panic("resp.StatusCode != http.StatusOK")
	}

	respJson, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		panic(err)
	}
	books := &[]*handler.Book{}
	err = json.Unmarshal(respJson, books)
	if err != nil {
		panic(err)
	}
	for _, b := range *books {
		fmt.Printf("%+v", b)
	}
}
