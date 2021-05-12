package main

import (
	"bytes"
	"crypto/aes"
	"crypto/cipher"
	"log"
)

func main() {
	msg := []byte(`[{"EsdEmsId": "hz10001","ShopName": "工厂","LineName": "车间","StationName": "工位","ProductName": "","EsdValue": 0,"EmsCreateTime": "2020-01-01 00:00:00","EsdDeviceId": "0001","LineStatus": 0,"ESDType": 0}]`)
	key := []byte{0x12, 0x30, 0x1a, 0x45, 0x7d, 0x1e, 0x90, 0x71, 0x10, 0x2a, 0x5f, 0x6c, 0x2c, 0x40, 0x81, 0xaa}
	iv := []byte{0x12, 0x30, 0x1a, 0x46, 0x7a, 0x1e, 0x90, 0x71, 0x10, 0x2a, 0x5f, 0x6c, 0x2c, 0x40, 0x81, 0xaa}
	block, err := aes.NewCipher(key)
	if err != nil {
		log.Fatal(err)
	}
	cbc := cipher.NewCBCEncrypter(block, iv)
	l := len(msg)
	pl := 16 - l%16
	msg = append(msg, bytes.Repeat([]byte{byte(pl)}, pl)...)
	ct := make([]byte, len(msg))
	cbc.CryptBlocks(ct, msg)
	log.Printf("%X", ct)
}
