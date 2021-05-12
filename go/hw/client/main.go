package main

import (
	"bytes"
	"context"
	"crypto/aes"
	"crypto/cipher"
	"crypto/hmac"
	"crypto/rand"
	"crypto/rsa"
	"crypto/sha1"
	"crypto/sha512"
	"encoding/base64"
	"flag"
	"fmt"
	"io/ioutil"
	"log"
	"math/big"
	"net/http"
	"net/http/httptrace"
	"time"
	mrand "math/rand"
	// "crypto/x509"
	// "encoding/pem"
)

func WroteHeaderField(key string, value []string) {
	log.Println(key, value)
}

func init() {
    mrand.Seed(time.Now().UnixNano())
}

var letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"

func randStr(n int) string {
    b := make([]byte, n)
    for i := range b {
        b[i] = letters[mrand.Intn(len(letters))]
    }
    return string(b)
}

func getPublicKey() *rsa.PublicKey {
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

	log.Println(pub.N, pub.E)

	return pub
// 	pemStr := `
// -----BEGIN PUBLIC KEY-----
// MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDB3BSKCsw3sxemGqRKvcwHIfDb
// EILbOeaNoKJv3EAeYSoZ9r3bBzWw5L13YryCKFOYqzUd7ToC0vx7Ccsmlwjo5Wiw
// Qn1DA/L1XolKTZRfKWZgsdPPHuOoCTQUP+vhx4uTkpRyFdAKPYB1d2SiWE8kef+b
// AbJkU3VI9BD1XIYuOwIDAQAB
// -----END PUBLIC KEY-----`
// 	block, _ := pem.Decode([]byte(pemStr))
// 	if block == nil {
// 		log.Fatal("failed to parse PEM block containing the key")
// 	}

// 	pub, err := x509.ParsePKIXPublicKey(block.Bytes)
// 	if err != nil {
// 		log.Fatal(err)
// 	}
// 	log.Println(pub.(*rsa.PublicKey).N, pub.(*rsa.PublicKey).E)
// 	return pub.(*rsa.PublicKey)
}

func main() {
	appId := flag.String("appId", "0123", "")
	url := flag.String("url", "http://localhost:1123/hawkeye", "")
	flag.Parse()

	log.SetFlags(log.LstdFlags | log.Lshortfile)

	timestamp := time.Now().Unix()

	echostr := randStr(16)

	dataJson := `[{"EsdEmsId": "hz10001","ShopName": "工厂","LineName": "车间","StationName": "工位","ProductName": "","EsdValue": 0,"EmsCreateTime": "2020-01-01 00:00:00","EsdDeviceId": "0001","LineStatus": 0,"ESDType": 0}]`

	signStr := fmt.Sprintf(
		"appId=%s&timestamp=%d&echostr=%s&dataJson=%s",
		*appId, timestamp, echostr, dataJson,
	)

	hmacKey := []byte{64, 74, 100, 194, 135, 153, 37, 10, 167, 68, 40, 10, 150, 173, 227, 106, 27, 32, 147, 180, 76, 199, 178, 199, 131, 145, 85, 133, 92, 143, 159, 154, 30, 146, 135, 25, 36, 128, 55, 179, 101, 91, 93, 101, 46, 64, 215, 193, 186, 117, 53, 175, 177, 238, 112, 159, 137, 168, 157, 30, 40, 81, 101, 57}
	hmac := hmac.New(sha512.New, hmacKey)
	hmac.Write([]byte(signStr))
	signature := base64.RawURLEncoding.EncodeToString(hmac.Sum(nil))

	urlQuery := fmt.Sprintf(
		"appId=%s&timestamp=%d&echostr=%s&signature=%s",
		*appId, timestamp, echostr, signature,
	)

	pub := getPublicKey()

	aesKey := make([]byte, 16)
	n, _ := rand.Reader.Read(aesKey)
	if n != 16 {
		log.Fatal("n != 16")
	}
	aesKeyEncrypted, err := rsa.EncryptOAEP(sha1.New(), rand.Reader, pub, aesKey, nil)
	if err != nil {
		log.Fatal(err)
	}

	aesIV := make([]byte, 16)
	n, _ = rand.Reader.Read(aesIV)
	if n != 16 {
		log.Fatal("n != 16")
	}

	buf := &bytes.Buffer{}
	buf.Write(aesKeyEncrypted)
	buf.Write(aesIV)

	block, err := aes.NewCipher(aesKey)
	if err != nil {
		log.Fatal(err)
	}
	cbc := cipher.NewCBCEncrypter(block, aesIV)
	dataBs := []byte(dataJson)
	l := len(dataBs)
	pl := 16 - l%16
	dataBs = append(dataBs, bytes.Repeat([]byte{byte(pl)}, pl)...)
	log.Printf("%x", dataBs)
	dataEncrypted := make([]byte, len(dataBs))
	cbc.CryptBlocks(dataEncrypted, dataBs)
	buf.Write([]byte{byte(pl)})
	buf.Write(dataEncrypted)

	trace := &httptrace.ClientTrace{WroteHeaderField: WroteHeaderField}
	ctx := httptrace.WithClientTrace(context.Background(), trace)
	log.Print(*url+"?"+urlQuery)
	req, err := http.NewRequest("POST", *url+"?"+urlQuery, buf)
	if err != nil {
		log.Fatal(err)
	}
	req = req.WithContext(ctx)
	req.Header.Add("Content-Type", "application/x-www-form-urlencoded")
	client := &http.Client{}
	resp, err := client.Do(req)
	defer resp.Body.Close()
	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		log.Fatal(err)
	}
	log.Print(resp)
	log.Print(string(body))
}
