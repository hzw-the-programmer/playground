package main

import (
	"fmt"
	"reflect"
	"runtime"
)

func haha() {
}

func main() {
	fmt.Println(runtime.FuncForPC(reflect.ValueOf(haha).Pointer()).Name())
}
