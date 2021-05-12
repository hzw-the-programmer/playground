package main

import (
	"crypto"
	"crypto/rand"
	"crypto/rsa"
	"crypto/sha256"
	"io/ioutil"
	"log"
)

func main() {
	log.SetFlags(log.LstdFlags | log.Lshortfile)

	d, err := ioutil.ReadFile("rsasign.go")
	if err != nil {
		log.Fatal(err)
	}
	hashed := sha256.Sum256(d)

	priv, err := rsa.GenerateKey(rand.Reader, 1024)
	if err != nil {
		log.Fatal(err)
	}
	pub := priv.Public()

	sig, err := rsa.SignPKCS1v15(rand.Reader, priv, crypto.SHA256, hashed[:])
	if err != nil {
		log.Fatal(err)
	}

	err = rsa.VerifyPKCS1v15(pub.((*rsa.PublicKey)), crypto.SHA256, hashed[:], sig)
	if err != nil {
		log.Fatal(err)
	}
}
