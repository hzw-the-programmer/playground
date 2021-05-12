package main

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"log"
)

func main() {
	log.SetFlags(log.LstdFlags | log.Lshortfile)

	key := [16]byte{}
	iv := [16]byte{}

	n, _ := rand.Reader.Read(key[:])
	if n != 16 {
		log.Fatal("n != 16")
	}

	n, _ = rand.Reader.Read(iv[:])
	if n != 16 {
		log.Fatal("n != 16")
	}

	plaintext := []byte("hello world")
	ciphertext := make([]byte, len(plaintext))

	block, err := aes.NewCipher(key[:])
	if err != nil {
		log.Fatal(err)
	}
	cfb := cipher.NewCFBEncrypter(block, iv[:])
	cfb.XORKeyStream(ciphertext, plaintext)

	cfbdec := cipher.NewCFBDecrypter(block, iv[:])
	plaintextCopy := make([]byte, len(ciphertext))
	cfbdec.XORKeyStream(plaintextCopy, ciphertext)

	log.Print(string(plaintextCopy))
}
