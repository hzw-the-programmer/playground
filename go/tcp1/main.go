package main

import (
	"flag"
	"log"

	"tcp1/lora"
)

func main() {
	addr := flag.String("addr", "localhost:25001", "lora server host:port")
	cmd := flag.String("cmd", "lc", "run lora server(ls) or client(lc)")
	flag.Parse()

	log.SetFlags(log.LstdFlags | log.Lshortfile)

	if *cmd == "ls" {
		lora.Serve(*addr)
	} else if *cmd == "lc" {
		lora.Connect(*addr)
	}
}
