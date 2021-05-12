package main

import (
	"fmt"

	"golang.org/x/text/language"
	"golang.org/x/text/message"
)

func init() {
	message.SetString(language.BritishEnglish, "%s went to %s.", "%s went to %s.")
	message.SetString(language.AmericanEnglish, "%s went to %s.", "%s is in %s.")
	message.SetString(language.Greek, "%s went to %s.", "%s πήγε στήν %s.")
}

func main() {
	tags := []language.Tag{
		language.BritishEnglish,
		language.AmericanEnglish,
		language.Greek,
	}
	for _, tag := range tags {
		p := message.NewPrinter(tag)
		p.Printf("%s went to %s.", "hzw", "England")
		fmt.Println()
	}
}
