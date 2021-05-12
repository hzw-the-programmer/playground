package main

import (
	"log"

	"github.com/BurntSushi/toml"
)

func main() {
	var data interface{}
	if _, err := toml.DecodeFile("./en.toml", &data); err != nil {
		panic(err)
	}
	log.Printf("%+v", data)
}
