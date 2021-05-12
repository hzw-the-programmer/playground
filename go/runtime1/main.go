package main

import (
	"log"
	"runtime"
)

func main() {
	pc, file, line, _ := runtime.Caller(0)
	f := runtime.FuncForPC(pc)
	log.Print(f.Name())
	log.Print(file)
	log.Print(line)
}
