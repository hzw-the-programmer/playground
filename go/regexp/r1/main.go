package main

import (
	"fmt"
	"regexp"
)

func main() {
	re := regexp.MustCompile(`a(x*)b(y|z)c`)
	fmt.Printf("%q\n", re.FindString("-axxxbyc-"))
	fmt.Printf("%q\n", re.FindString("-abzc-"))
	fmt.Printf("%q\n", re.FindStringSubmatch("-axxxbyc-"))
	fmt.Printf("%q\n", re.FindStringSubmatch("-abzc-"))

	str := "-axxxbyc-"
	indexes := re.FindStringIndex(str)
	fmt.Printf("%q\n", str[indexes[0]:indexes[1]])

	str = "-abzc-"
	indexes = re.FindStringIndex(str)
	fmt.Printf("%q\n", str[indexes[0]:indexes[1]])
}
