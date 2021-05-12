package main

import (
	"bytes"
	"encoding/gob"
	"log"
)

type Device struct {
	Cpu string
	Os  string
}

func main() {
	d := &Device{
		Cpu: "arm",
		Os:  "hzw",
	}

	buf := &bytes.Buffer{}
	encoder := gob.NewEncoder(buf)
	if err := encoder.Encode(d); err != nil {
		log.Fatal(err)
	}

	d1 := &Device{}

	decoder := gob.NewDecoder(buf)
	if err := decoder.Decode(d1); err != nil {
		log.Fatal(err)
	}

	log.Printf("%+v", d1)
}
