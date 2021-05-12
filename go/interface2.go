package main

import (
	"fmt"
)

type I1 interface {
	M1()
	SetM1(p int)
}

type S1 struct {
	P1 int
}

func (s *S1) M1() {
	fmt.Println(s.P1)
}

func (s *S1) SetM1(p int) {
	s.P1 = p
}

type S2 struct {
	I1
}

func main() {
	s1 := S1{11}
	s1.M1()
	s1.SetM1(23)
	s1.M1()

	s2 := S2{&s1}
	s2.M1()
	s2.SetM1(11)
	s1.M1()
}