package main

import (
	"crypto/rsa"
	"crypto/x509"
	"encoding/pem"
	"io/ioutil"
	"log"
	"os"
)

// openssl genrsa -out pkcs1.pem 2048
func pkcs1() {
	f, err := os.Open("pkcs1.pem")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	c, err := ioutil.ReadAll(f)
	if err != nil {
		log.Fatal(err)
	}

	block, rest := pem.Decode(c)
	log.Print(rest)
	if block == nil {
		return
	}
	log.Print(block.Type)
	log.Print(block.Headers)

	priKey, err := x509.ParsePKCS1PrivateKey(block.Bytes)
	if err != nil {
		log.Fatal(err)
	}
	log.Print("priKey.PublicKey.N")
	log.Print(priKey.PublicKey.N)
	log.Print("priKey.PublicKey.E")
	log.Print(priKey.PublicKey.E)
	log.Print("priKey.D")
	log.Print(priKey.D)
	log.Print("priKey.Primes")
	for _, prime := range priKey.Primes {
		log.Print(prime)
	}
	log.Print("priKey.Precomputed.Dp")
	log.Print(priKey.Precomputed.Dp)
	log.Print("priKey.Precomputed.Dq")
	log.Print(priKey.Precomputed.Dq)
	log.Print("priKey.Precomputed.Qinv")
	log.Print(priKey.Precomputed.Qinv)
	log.Print("priKey.Precomputed.CRTValues")
	for _, cRTValue := range priKey.Precomputed.CRTValues {
		log.Print(cRTValue)
	}
}

// openssl pkcs8 -topk8 -inform PEM -outform PEM -nocrypt -in pkcs1.pem -out pkcs8.pem
func pkcs8() {
	f, err := os.Open("pkcs8.pem")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	c, err := ioutil.ReadAll(f)
	if err != nil {
		log.Fatal(err)
	}

	block, rest := pem.Decode(c)
	log.Print(rest)
	if block == nil {
		return
	}
	log.Print(block.Type)
	log.Print(block.Headers)

	priKey, err := x509.ParsePKCS8PrivateKey(block.Bytes)
	if err != nil {
		log.Fatal(err)
	}
	rsaPriKey, ok := priKey.(*rsa.PrivateKey)
	log.Print(ok)
	log.Print("rsaPriKey.PublicKey.N")
	log.Print(rsaPriKey.PublicKey.N)
	log.Print("rsaPriKey.PublicKey.E")
	log.Print(rsaPriKey.PublicKey.E)
	log.Print("rsaPriKey.D")
	log.Print(rsaPriKey.D)
	log.Print("rsaPriKey.Primes")
	for _, prime := range rsaPriKey.Primes {
		log.Print(prime)
	}
	log.Print("rsaPriKey.Precomputed.Dp")
	log.Print(rsaPriKey.Precomputed.Dp)
	log.Print("rsaPriKey.Precomputed.Dq")
	log.Print(rsaPriKey.Precomputed.Dq)
	log.Print("rsaPriKey.Precomputed.Qinv")
	log.Print(rsaPriKey.Precomputed.Qinv)
	log.Print("rsaPriKey.Precomputed.CRTValues")
	for _, cRTValue := range rsaPriKey.Precomputed.CRTValues {
		log.Print(cRTValue)
	}
}

// openssl rsa -in pkcs1.pem -pubout -out pub.pem
func pub() {
	f, err := os.Open("pub.pem")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	c, err := ioutil.ReadAll(f)
	if err != nil {
		log.Fatal(err)
	}

	block, rest := pem.Decode(c)
	log.Print(rest)
	if block == nil {
		return
	}
	log.Print(block.Type)
	log.Print(block.Headers)

	pk, err := x509.ParsePKIXPublicKey(block.Bytes)
	if err != nil {
		log.Fatal(err)
	}

	rsaPubKey, ok := pk.(*rsa.PublicKey)
	log.Print(ok)
	log.Print(rsaPubKey.N)
	log.Print(rsaPubKey.E)
}

func crt() {
	f, err := os.Open("localhost.crt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	c, err := ioutil.ReadAll(f)
	if err != nil {
		log.Fatal(err)
	}

	block, rest := pem.Decode(c)
	log.Print(rest)
	if block == nil {
		return
	}
	log.Print(block.Type)
	log.Print(block.Headers)

	crt, err := x509.ParseCertificate(block.Bytes)
	if err != nil {
		log.Fatal(err)
	}

	rsaPubKey, ok := crt.PublicKey.(*rsa.PublicKey)
	log.Print(ok)
	log.Print(rsaPubKey.N)
	log.Print(rsaPubKey.E)
}

func main() {
	pkcs1()
	log.Println()
	pkcs8()
	log.Println()
	pub()
	log.Println()
	crt()
}
