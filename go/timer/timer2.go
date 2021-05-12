package main

import (
	"log"
	"time"
)

func main() {
	test1()
	test2()
	test3()
	test4()
	test5()
}

func test1() {
	t := time.NewTimer(time.Second)
	time.Sleep(500 * time.Millisecond)
	s := time.Now()
	<-t.C
	log.Print(time.Now().Sub(s))
	log.Print(t.Stop())
}

func test2() {
	t := time.NewTimer(time.Second)
	time.Sleep(500 * time.Millisecond)
	t.Reset(time.Second)
	s := time.Now()
	<-t.C
	log.Print(time.Now().Sub(s))
	log.Print(t.Stop())
}

func test3() {
	t := time.NewTimer(time.Second)
	time.Sleep(500 * time.Millisecond)
	log.Print(t.Stop())
}

func test4() {
	s := time.Now()
	t := time.NewTimer(500 * time.Millisecond)
	<-t.C
	t.Reset(500 * time.Millisecond)
	<-t.C
	log.Print(time.Now().Sub(s))
}

func test5() {
	t := time.NewTimer(500 * time.Millisecond)
	log.Print(t.Stop())
	t.Reset(500 * time.Millisecond)
	s := time.Now()
	<-t.C
	log.Print(time.Now().Sub(s))
}
