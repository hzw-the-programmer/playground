package main

import (
	"log"
)

func change(m map[string]int) {
	m["h"] = 1
	m["z"] = 2
	m["w"] = 3
}

func main() {
	m := map[string]int{"h": 0}
	log.Print(m)
	change(m)
	log.Print(m)
}
