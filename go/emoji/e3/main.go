package main

import (
	"bytes"
	"fmt"

	"github.com/go-xman/go.emoji/official"
)

func ReplaceEmoji(str string) string {
	buf := bytes.Buffer{}

	found := false
	nextIndex := 0
	for i, r := range str {
		if i < nextIndex {
			continue
		}

		match, length := official.AllSequences.HasEmojiPrefix(str[i:])
		if !match {
			found = false
			buf.WriteRune(r)
			continue
		}

		nextIndex = i + length

		if !found {
			found = true
			buf.WriteString("[hzw]")
		}
	}

	return buf.String()
}

func main() {
	str := "🙂aaa🙂🙂bbb🙂🙂🙂ccc🙂🙂🙂🙂ddd🙂🙂🙂🙂🙂"

	fmt.Print(ReplaceEmoji(str))
}
