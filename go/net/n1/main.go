package main

import (
	"fmt"
	"net"
)

func main() {
	ifs, err := net.Interfaces()
	if err != nil {
		panic(err)
	}
	for _, i := range ifs {
		fmt.Printf("%+v\n", i)
	}
}
