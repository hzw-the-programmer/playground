package main

import (
	"strings"

	"log"
)

func main() {
	s := "feature_phone.pb-c.obj"
	log.Print(strings.Contains(s, "pb-c"))
}
