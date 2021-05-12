package main

import (
	"crypto/rand"
	"crypto/rsa"
	"crypto/sha1"
	"log"
	"fmt"
	"encoding/pem"
	"crypto/x509"
)

func getPublicKey() *rsa.PublicKey {
	// ns := "yayxrS87mp3L4uwSAiV2gg/j6Pot4KnA+FG2+Lv94PH7KW7QyEnYf/vgHLJO4+1PH6wt+FUul9/fGN9fVC5+VxdmREZI0U5X2TjaomZ0diVS/XMlRbsCn+0ZJ954dg8b0H3aos4g9O5662lAeVSFOswyzHqdKMAOVOXH31h4LKs="
	// es := "AQAB"

	// nbs, err := base64.StdEncoding.DecodeString(ns)
	// if err != nil {
	// 	log.Fatal(err)
	// }

	// ebs, err := base64.StdEncoding.DecodeString(es)
	// if err != nil {
	// 	log.Fatal(err)
	// }

	// n := new(big.Int).SetBytes(nbs)
	// e := new(big.Int).SetBytes(ebs)

	// return &rsa.PublicKey{N: n, E: int(e.Int64())}
	pemStr := `
-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDJrLGtLzuancvi7BICJXaCD+Po
+i3gqcD4Ubb4u/3g8fspbtDISdh/++Acsk7j7U8frC34VS6X398Y319ULn5XF2ZE
RkjRTlfZONqiZnR2JVL9cyVFuwKf7Rkn3nh2DxvQfdqiziD07nrraUB5VIU6zDLM
ep0owA5U5cffWHgsqwIDAQAB
-----END PUBLIC KEY-----`
	block, _ := pem.Decode([]byte(pemStr))
	if block == nil {
		log.Fatal("failed to parse PEM block containing the key")
	}

	pub, err := x509.ParsePKIXPublicKey(block.Bytes)
	if err != nil {
		log.Fatal(err)
	}
	return pub.(*rsa.PublicKey)
}

func main() {
	aesKey := []byte{0x12, 0x30, 0x1a, 0x45, 0x7d, 0x1e, 0x90, 0x71, 0x10, 0x2a, 0x5f, 0x6c, 0x2c, 0x40, 0x81, 0xaa}
	pub := getPublicKey()
	aesKeyEncrypted, err := rsa.EncryptOAEP(sha1.New(), rand.Reader, pub, aesKey, nil)
	if err != nil {
		log.Fatal(err)
	}
	for _, b := range aesKeyEncrypted {
		fmt.Print(b, ", ")
	}
}
