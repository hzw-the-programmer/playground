package main

import (
	"bytes"
	"fmt"
	"regexp"
)

func main() {
	match, _ := regexp.MatchString("p([a-z]+)ch", "hello, peach")
	fmt.Println(match)

	r, _ := regexp.Compile("p([a-z]+)ch")

	fmt.Println(r.MatchString("hello, peach"))

	fmt.Println(r.FindString("hello, peach, punch"))
	fmt.Println(r.FindStringIndex("hello, peach, punch"))
	fmt.Println(r.FindStringSubmatch("hello, peach, punch"))
	fmt.Println(r.FindStringSubmatchIndex("hello, peach, punch"))

	fmt.Println(r.FindAllString("hello, peach, punch, pinch", -1))
	fmt.Println(r.FindAllString("hello, peach, punch, pinch", 2))
	fmt.Println(r.FindAllStringIndex("hello, peach, punch, pinch", -1))
	fmt.Println(r.FindAllStringSubmatch("hello, peach, punch, pinch", -1))
	fmt.Println(r.FindAllStringSubmatchIndex("hello, peach, punch, pinch", -1))

	fmt.Println(r.Match([]byte("hello, peach, punch, pinch")))

	r = regexp.MustCompile("p([a-z]+)ch")
	fmt.Println(r)

	fmt.Println(r.ReplaceAllString("hello, peach, punch, pinch", "<fruit>"))

	in := []byte("hello, peach, punch, pinch")
	out := r.ReplaceAllFunc(in, bytes.ToUpper)
	fmt.Println(string(out))
}
