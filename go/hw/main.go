package main

import (
	"crypto/rand"
	"crypto/rsa"
	"crypto/sha256"
	"encoding/base64"
	"log"
	"math/big"
	// "net/http/httptrace"
	// "context"
	"bytes"
)

func WroteHeaderField(key string, value []string) {
	log.Println(key, value)
}

func main() {
	log.SetFlags(log.Lshortfile)

	ns := "wdwUigrMN7MXphqkSr3MByHw2xCC2znmjaCib9xAHmEqGfa92wc1sOS9d2K8gihTmKs1He06AtL8ewnLJpcI6OVosEJ9QwPy9V6JSk2UXylmYLHTzx7jqAk0FD/r4ceLk5KUchXQCj2AdXdkolhPJHn/mwGyZFN1SPQQ9VyGLjs="
	es := "AQAB"

	nbs, err := base64.StdEncoding.DecodeString(ns)
	if err != nil {
		log.Fatal(err)
	}

	ebs, err := base64.StdEncoding.DecodeString(es)
	if err != nil {
		log.Fatal(err)
	}

	n := new(big.Int).SetBytes(nbs)
	e := new(big.Int).SetBytes(ebs)

	pub := &rsa.PublicKey{N: n, E: int(e.Int64())}

	buf := &bytes.Buffer{}

	aesKey := [16]byte{}
	_, err = rand.Reader.Read(aesKey[:])
	if err != nil {
		log.Fatal(err)
	}
	aesKeyEncrypted, err := rsa.EncryptOAEP(sha256.New(), rand.Reader, pub, aesKey[:], nil)
	if err != nil {
		log.Fatal(err)
	}

	aesIV := [16]byte{}
	_, err = rand.Reader.Read(aesIV[:])
	if err != nil {
		log.Fatal(err)
	}

	buf.Write(aesKeyEncrypted)
	buf.Write(aesIV[:])
	log.Print(buf.Len())
	// trace := &httptrace.ClientTrace{WroteHeaderField: WroteHeaderField}
	// ctx := httptrace.WithClientTrace(context.Background(), trace)

	// msg := "hello world"

	// bs, err := rsa.EncryptOAEP(sha256.New(), rand.Reader, pub, []byte(msg), nil)
	// if err != nil {
	// 	log.Fatal(err)
	// }
	// log.Print(bs)
}
