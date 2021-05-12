package main

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"io/ioutil"
	"log"
)

func main() {
	log.SetFlags(log.LstdFlags | log.Lshortfile)

	plaintext, err := ioutil.ReadFile("aesgcm.go")
	if err != nil {
		log.Fatal(err)
	}

	key := make([]byte, 16)
	if n, _ := rand.Reader.Read(key); n != 16 {
		log.Fatal("n != 16")
	}
	block, err := aes.NewCipher(key)
	if err != nil {
		log.Fatal(err)
	}
	gcm, err := cipher.NewGCM(block)
	if err != nil {
		log.Fatal(err)
	}
	nonceSize := gcm.NonceSize()
	nonce := make([]byte, nonceSize)
	if n, _ := rand.Reader.Read(nonce); n != nonceSize {
		log.Fatal("n != nonceSize")
	}
	ciphertext := gcm.Seal(nil, nonce, plaintext, nil)

	plaintext, err = gcm.Open(nil, nonce, ciphertext, nil)
	if err != nil {
		log.Fatal(err)
	} else {
		log.Print(string(plaintext))
	}
}
