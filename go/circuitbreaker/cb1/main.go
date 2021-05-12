package main

import (
	"errors"
	"log"
	"fmt"
	"time"

	"github.com/rubyist/circuitbreaker"
)

func main() {
	cb := circuit.NewThresholdBreaker(3)
	
	events := cb.Subscribe()
	go func() {
		for e := range events {
			log.Print(BreakerEvent(e))
		}
	}()
	
	i := 0
	for ;; {
		log.Print("return: ", cb.Call(func() error {
			log.Print(i)
			return errors.New(fmt.Sprintf("error %d", i))
		}, 0))
		i += 1
		time.Sleep(1*time.Second)
	}
}

type BreakerEvent circuit.BreakerEvent

func (e BreakerEvent) String() string {
	switch circuit.BreakerEvent(e) {
	case circuit.BreakerTripped:
		return "BreakerTripped"
	case circuit.BreakerReset:
		return "BreakerReset"
	case circuit.BreakerFail:
		return "BreakerFail"
	case circuit.BreakerReady:
		return "BreakerReady"
	}
	return "unknown"
}
