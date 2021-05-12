package main

import (
	"log"
)

func main() {
	var m1 map[string]int
	log.Println("m1 == nil", m1 == nil)
	score, ok := m1["hzw"]
	log.Print(ok, score)

	delete(m1, "hzw")
	score, ok = m1["hzw"]
	log.Print(ok, score)

	m1 = make(map[string]int)
	m1["hzw"] = 1
	score, ok = m1["hzw"]
	log.Print(ok, score)

	delete(m1, "hzw")
	score, ok = m1["hzw"]
	log.Print(ok, score)

	log.Print()

	m2 := map[string]int{}
	log.Println("m2 == nil", m2 == nil)
	score, ok = m2["hzw"]
	log.Print(ok, score)
	m2["hzw"] = 1
	score, ok = m2["hzw"]
	log.Print(ok, score)

	log.Print()

	var b1 []byte
	log.Println("b1 == nil", b1 == nil, b1, len(b1))
	b1 = append(b1, 0x00)
	log.Print(b1 == nil, b1, len(b1))

	log.Print()

	b2 := []byte{}
	log.Println("b2 == nil", b2 == nil, b2, len(b2))
	b2 = append(b2, 0x00)
	log.Print(b2 == nil, b2, len(b2))
}
