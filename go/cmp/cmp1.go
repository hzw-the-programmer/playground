package main

import (
	"log"
	"reflect"
)

type A struct {
	f1 string
	f2 int
}

type B struct {
	f1 int
	f2 []byte
}

func print(equal bool) {
	if equal {
		log.Print("equal")
	} else {
		log.Print("not equal")
	}
}

func main() {
	a := A{"hello", 1}
	b := A{"world", 2}
	print(a == b)
	a.f1 = "world"
	a.f2 = 2
	print(a == b)

	c := [2]A{{"hello", 1}, {"world", 2}}
	d := [2]A{{"world", 1}, {"hello", 2}}
	print(c == d)
	d[0].f1 = "hello"
	d[1].f1 = "world"
	print(c == d)

	e := []A{{"hello", 1}, {"world", 2}}
	f := []A{{"world", 1}, {"hello", 2}}
	//print(e == f)
	print(reflect.DeepEqual(e, f))
	f[0].f1 = "hello"
	f[1].f1 = "world"
	//print(e == f)
	print(reflect.DeepEqual(e, f))

	g := B{123, []byte{0x01, 0x02}}
	h := B{124, []byte{0x03, 0x04}}
	//print(g == h)
	print(reflect.DeepEqual(g, h))
	h.f1 = 123
	h.f2[0] = 0x01
	h.f2[1] = 0x02
	//print(g == h)
	print(reflect.DeepEqual(g, h))
}
