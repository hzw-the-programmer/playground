package main

import (
	"encoding/base64"
	"encoding/hex"
	"encoding/json"
	"encoding/binary"
	"log"

	"github.com/npat-efault/crc16"
)

type A struct {
	F1 string `json:"f1"`
	F2 []byte `json:"f2"`
}

func main() {
	a := &A{"haha", []byte{0x53, 0x54}}
	str, err := json.Marshal(a)
	if err != nil {
		log.Fatal(err)
	}
	log.Println(string(str))
	var b A
	err = json.Unmarshal(str, &b)
	if err != nil {
		log.Fatal(err)
	}
	log.Println(b.F2)
	log.Println(hex.EncodeToString(b.F2))
	log.Println(string(b.F2))

	bs, err := base64.StdEncoding.DecodeString("QVVVQUVwQT0=")
	bs, err = base64.StdEncoding.DecodeString(string(bs))
	log.Println(hex.EncodeToString(bs))
	log.Println(binary.LittleEndian.Uint16(bs[3:]))
	log.Println(crc16.Checksum(crc16.Modbus, bs[:3]))
}
