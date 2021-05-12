package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Printf("Usage: %s file\n", os.Args[0])
		return
	}

	fn := os.Args[1]
	file, err := os.Open(fn)
	if err != nil {
		log.Fatal(err)
	}
	s := bufio.NewScanner(file)
	for s.Scan() {
		line := strings.TrimSpace(s.Text())
		if !strings.HasPrefix(line, "\"value\": \"") {
			continue
		}
		//emoji := strings.Trim(line, "\"value: ,")
		//fmt.Printf("%s %d %+q\n", emoji, len(emoji), emoji)
		fmt.Printf("%+q\n", line)
	}
}
