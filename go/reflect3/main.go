package main

import (
	"log"
	"reflect"
)

type A struct{}

func (A) F1()  {}
func (A) f1()  {}
func (*A) F2() {}
func (*A) f2() {}

func info(t reflect.Type) {
	log.Print("Name:", t.Name())
	log.Print("PkgPath:", t.PkgPath())
	log.Print("String:", t.String())
	log.Print("Kind:", t.Kind())
	log.Print("NumMethod:", t.NumMethod())
	for i := 0; i < t.NumMethod(); i++ {
		m := t.Method(i)
		log.Print(m.Name)
		log.Print(m.PkgPath)
		log.Print(m.Type.Kind())
		log.Print(m.Func.Kind() == m.Type.Kind())
		log.Print(m.Index)
	}
	if t.Kind() == reflect.Ptr {
		info(t.Elem())
	}
}

func main() {
	t1 := reflect.TypeOf(A{})
	t2 := reflect.TypeOf(&A{})
	info(t1)
	log.Print()
	info(t2)
	log.Print(t1 == t2)
	log.Print(t1 == t2.Elem())
}
