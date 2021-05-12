package main

import (
	"flag"
	"fmt"
	"strings"
)

type sheets []string

func (s *sheets) String() string {
	fmt.Println("sheets.String")
	return "hhhh"
}

func (s *sheets) Set(str string) error {
	fmt.Println("sheets.Set: " + str)
	strs := strings.Split(str, ",")
	*s = append(*s, strs...)
	return nil
}

func main() {
	s := sheets{}
	flag.Var(&s, "sheet", "set sheet")
	flag.Parse()
	for _, str := range s {
		fmt.Println(str)
	}
}
