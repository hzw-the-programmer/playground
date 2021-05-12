package main

import (
	"bytes"
	"encoding/binary"
	"fmt"
	"reflect"
)

// type Integer uint32
type Integer uint16

func main() {
	var i Integer = 1
	buf := &bytes.Buffer{}
	binary.Write(buf, binary.BigEndian, &i)
	fmt.Println(buf.Bytes())

	var d interface{} = &i
	v := reflect.ValueOf(d)
	fmt.Printf("%T %s\n", v, v)
	fmt.Printf("%T %s\n", v.Kind(), v.Kind())
	fmt.Printf("%T %s\n", v.Type(), v.Type())
	fmt.Println(v.Type().Kind())
	v = v.Elem()
	fmt.Printf("%T %v\n", v.Kind(), v.Kind())
	fmt.Printf("%T %v\n", v.Type(), v.Type())
	fmt.Println(v.Type().Kind())
}
