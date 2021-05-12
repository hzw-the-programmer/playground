package main

import (
	"bytes"
	"encoding/hex"
	"log"
)

func main() {
	var bb bytes.Buffer
	e := hex.NewEncoder(&bb)
	e.Write([]byte{0x01, 0x02, 0x03, 0x04})
	log.Print(bb.Bytes())
	log.Print(string(bb.Bytes()))

	bb1 := bytes.NewBufferString("01020304")
	d := hex.NewDecoder(bb1)
	bs := make([]byte, 4)
	d.Read(bs)
	log.Print(bs)
}
