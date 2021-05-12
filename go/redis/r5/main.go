package main

import (
	"crypto/rand"
	"encoding/base64"
	"io"
	"log"
	"os"

	"github.com/gorilla/securecookie"
)

func main() {
	// authKey, _ := genKey(32)
	// encKey, _ := genKey(32)
	// log.Print(authKey)
	// log.Print(encKey)

	name := os.Args[1]
	value := os.Args[2]
	hashKeyBase64 := os.Args[3]
	blockKeyBase64 := os.Args[4]

	log.Print(name)
	log.Print(value)
	log.Print(hashKeyBase64)
	log.Print(blockKeyBase64)

	hashKey, err := base64.StdEncoding.DecodeString(hashKeyBase64)
	if err != nil {
		log.Fatal(err)
	}

	blockKey, err := base64.StdEncoding.DecodeString(blockKeyBase64)
	if err != nil {
		log.Fatal(err)
	}

	name = "hzw"
	value = "KXAE4F67GOCVTLVX3Y6O3OBZUYZDDAG5EDCKC53AIWPCT5QPROSA"

	s := securecookie.New(hashKey, blockKey)
	r, err := s.Encode(name, value)
	if err != nil {
		log.Fatal(err)
	}
	var v string
	err = s.Decode(name, r, &v)
	if err != nil {
		log.Fatal(err)
	}
	if value != v {
		log.Fatal("value != v")
	}
}

func genKey(len int) (string, error) {
	key := make([]byte, len)
	_, err := io.ReadFull(rand.Reader, key)
	if err != nil {
		return "", err
	}
	return base64.StdEncoding.EncodeToString(key), nil
}
