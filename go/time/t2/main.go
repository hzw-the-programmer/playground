package main

import (
	"fmt"
	"time"
)

func main() {
	t := time.Now()
	shanghai := t.In(time.FixedZone("", 8*60*60))
	newYork := t.In(time.FixedZone("", -4*60*60))
	london := t.In(time.FixedZone("UTC +1", 1*60*60))
	tokyo := t.In(time.FixedZone("Utc +9", 9*60*60))
	fmt.Println(shanghai.Format("02/01/2006 15:04:05.999999999 -0700 MST"))
	fmt.Println(newYork.Format("02/01/2006 15:04:05.999999999 -0700 MST"))
	fmt.Println(london.Format("02/01/2006 15:04:05.999999999 -0700 MST"))
	fmt.Println(tokyo.Format("02/01/2006 15:04:05.999999999 -0700 MST"))
}
