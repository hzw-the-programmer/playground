package main

import (
	"example.com/greetings"
	"fmt"
)

func main() {
	messages := greetings.Hello("Gladys")
	fmt.Println(messages)
}
