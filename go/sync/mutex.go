package main

import (
	"log"
	"sync"
	"time"
)

type logger struct {
	sync.Mutex
	s string
}

func (l *logger) print() {
	l.Lock()
	defer l.Unlock()

	l.Unlock()
	time.Sleep(2 * time.Second)
	l.Lock()

	log.Print(l.s)
}

func (l *logger) setS(s string) {
	log.Print("setS: try lock")
	l.Lock()
	log.Print("setS: lock success")
	defer l.Unlock()
	l.s = s
}

func main() {
	var wg sync.WaitGroup
	l := logger{s: "ok"}
	wg.Add(1)
	go func() {
		defer wg.Done()
		time.Sleep(1*time.Second)
		l.setS("not ok")
	}()
	l.print()
	wg.Wait()
}
