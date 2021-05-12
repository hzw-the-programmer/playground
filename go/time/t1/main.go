package main

import (
	"fmt"
	"time"
)

func main() {
	t := time.Now()
	fmt.Println(t.Location())
	fmt.Println(t.String())
	fmt.Println(t.Format(time.ANSIC))
	fmt.Println(t.Format(time.UnixDate))
	fmt.Println(t.Format(time.RubyDate))
	fmt.Println(t.Format(time.RFC822))
	fmt.Println(t.Format(time.RFC822Z))
	fmt.Println(t.Format(time.RFC850))
	fmt.Println(t.Format(time.RFC1123))
	fmt.Println(t.Format(time.RFC1123Z))
	fmt.Println(t.Format(time.RFC3339))
	fmt.Println(t.Format(time.RFC3339Nano))
	fmt.Println(t.Format(time.Kitchen))
	fmt.Println(t.Format(time.Stamp))
	fmt.Println(t.Format(time.StampMilli))
	fmt.Println(t.Format(time.StampMicro))
	fmt.Println(t.Format(time.StampNano))
	fmt.Println(t.AddDate(0, 0, -1).Format(time.RFC1123Z))

	fmt.Println(FormatTime(time.Now()))
	fmt.Println(FormatTime(time.Now().AddDate(0, 0, -1)))
	fmt.Println(FormatTime(time.Now().AddDate(0, 0, -2)))
	fmt.Println(FormatTime(time.Now().AddDate(-1, 0, 0)))
}

func FormatTime(t time.Time) string {
	today := time.Now()
	yesterday := today.AddDate(0, 0, -1)
	
	todayStart := time.Date(today.Year(), today.Month(), today.Day(), 0, 0, 0, 0, today.Location())
	todayEnd := time.Date(today.Year(), today.Month(), today.Day(), 23, 59, 59, 0, today.Location())
	
	yesterdayStart := time.Date(yesterday.Year(), yesterday.Month(), yesterday.Day(), 0, 0, 0, 0, yesterday.Location())
	yesterdayEnd := time.Date(yesterday.Year(), yesterday.Month(), yesterday.Day(), 23, 59, 59, 0, yesterday.Location())

	yearStart := time.Date(today.Year(), 1, 1, 0, 0, 0, 0, today.Location())
	yearEnd := time.Date(today.Year(), 12, 31, 23, 59, 59, 0, today.Location())

	if t.After(todayStart) && t.Before(todayEnd) {
		return t.Format("15:04")
	} else if t.After(yesterdayStart) && t.Before(yesterdayEnd) {
		return "yesterday " + t.Format("15:04")
	} else if t.After(yearStart) && t.Before(yearEnd) {
		return t.Format("02/01")
	}

	return t.Format("2006/02/01")
}
