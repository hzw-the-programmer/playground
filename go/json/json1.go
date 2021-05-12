package main

import (
	"encoding/json"
	"fmt"
)

type Person struct {
	Name   string  `json:"name"`
	Age    int     `json:"age"`
	Weight float64 `json:"weight,omitempty"`
	Height float64 `json:"-"`
}

func main() {
	p := &Person{
		Name: "hzw",
		Height: 11.11,
	}
	pjson, err := json.Marshal(p)
	if err != nil {
		fmt.Println(err)
	}
	fmt.Println(string(pjson))

	var p1 Person
	err = json.Unmarshal([]byte("{\"name\": \"hzw\"}"), &p1)
	if err != nil {
		fmt.Println(err)
	}
	fmt.Println(p1)
}
