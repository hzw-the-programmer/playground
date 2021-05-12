package main

import (
	"fmt"
	"log"
	"reflect"
)

type A struct {
	a1 int
	a2 string `k1:"v1" k2:"v2"`
	A3 int
	A4 string
}

func (a *A) psm1() {
}

func (a *A) psm2(p string) (r string) {
	return ""
}

func (a *A) Psm1() {
}

func (a *A) Psm2(p string) (r string) {
	return ""
}

func (a A) sm1() {
}

func (a A) sm2(p string) (r string) {
	return ""
}

func (a A) Sm1() {
}

func (a A) Sm2(p string) (r string) {
	return ""
}

func methodInfo(prefix string, m reflect.Method) {
	log.Printf("%s.Name: %q", prefix, m.Name)
	log.Printf("%s.PkgPath: %q", prefix, m.PkgPath)
	mt := m.Type
	typeInfo(fmt.Sprintf("%s.Type", prefix), mt)
	log.Printf("%s.Type.NumIn(): %d", prefix, mt.NumIn())
	for i := 0; i < mt.NumIn(); i++ {
		if i != 0 {
			typeInfo(fmt.Sprintf("%s.Type.In(%d)", prefix, i), mt.In(i))
		}
	}
	log.Printf("%s.Type.NumOut(): %d", prefix, mt.NumOut())
	for i := 0; i < mt.NumOut(); i++ {
		typeInfo(fmt.Sprintf("%s.Type.Out(%d)", prefix, i), mt.Out(i))
	}
}

func fieldInfo(prefix string, f reflect.StructField) {
	log.Printf("%s.Name: %q", prefix, f.Name)
	log.Printf("%s.PkgPath: %q", prefix, f.PkgPath)
	log.Printf("%s.Tag: %q", prefix, f.Tag)
	typeInfo(fmt.Sprintf("%s.Type", prefix), f.Type)
}

func typeInfo(prefix string, t reflect.Type) {
	k := t.Kind()
	log.Printf("%s.Kind().String(): %q", prefix, k)
	log.Printf("%s.Name(): %q", prefix, t.Name())
	log.Printf("%s.PkgPath(): %q", prefix, t.PkgPath())
	log.Printf("%s.String(): %q", prefix, t)

	log.Printf("%s.NumMethod(): %d", prefix, t.NumMethod())

	for i := 0; i < t.NumMethod(); i++ {
		methodInfo(fmt.Sprintf("%s.Method(%d)", prefix, i), t.Method(i))
	}

	if k == reflect.Struct {
		log.Printf("%s.NumField: %d", prefix, t.NumField())

		for i := 0; i < t.NumField(); i++ {
			fieldInfo(fmt.Sprintf("%s.Field(%d)", prefix, i), t.Field(i))			
		}
	} else if k == reflect.Ptr {
		typeInfo(fmt.Sprintf("%s.Elem()", prefix), t.Elem())
	}
}

func main() {
	var t reflect.Type
	
	t = reflect.TypeOf(A{})
	typeInfo("TypeOf(A{})", t)

	fmt.Println("")

	t = reflect.TypeOf(&A{})
	typeInfo("TypeOf(&A{})", t)

	// log.Print("******")

	// a := [2]byte{}
	// t = reflect.TypeOf(a)
	// log.Print(t.Name())
	// log.Print(t.PkgPath())
	// log.Print(t)
	// log.Print(t.Kind())
	// log.Print(t.Len())
	// log.Print(t.Elem().Kind())
	// log.Print(t.NumMethod())

	// log.Print("******")

	// sl := []float32{}
	// t = reflect.TypeOf(sl)
	// log.Print(t.Name())
	// log.Print(t.PkgPath())
	// log.Print(t)
	// log.Print(t.Kind())
	// log.Print(t.Elem().Kind())
	// log.Print(t.NumMethod())

	// log.Print("******")

	// f := func(a int, b A, f ...int) (c int, d *A, e string) { return 0, nil, "" }
	// t = reflect.TypeOf(f)
	// log.Print(t.Name())
	// log.Print(t.PkgPath())
	// log.Print(t)
	// log.Print(t.Kind())
	// log.Print(t.NumIn())
	// log.Print(t.NumOut())
	// log.Print(t.In(2).Kind())
	// log.Print(t.In(2).Elem().Kind())
	// log.Print(t.In(2).Elem().Bits())
	// log.Print(t.Out(1).Kind())
	// log.Print(t.Out(1).Elem().Kind())
	// log.Print(t.IsVariadic())
	// log.Print(t.NumMethod())

	// log.Print("******")

	// m := map[A]string{}
	// t = reflect.TypeOf(m)
	// log.Print(t.Name())
	// log.Print(t.PkgPath())
	// log.Print(t)
	// log.Print(t.Kind())
	// log.Print(t.Key().Kind())
	// log.Print(t.Elem().Kind())
	// log.Print(t.NumMethod())

	// log.Print("******")

	// //ci := make(chan<- A)
	// //ci := make(<-chan A)
	// ci := make(chan A)
	// t = reflect.TypeOf(ci)
	// log.Print(t.Name())
	// log.Print(t.PkgPath())
	// log.Print(t)
	// log.Print(t.Kind())
	// log.Print(t.Elem().Kind())
	// log.Print(t.ChanDir())
	// log.Print(t.NumMethod())
}
