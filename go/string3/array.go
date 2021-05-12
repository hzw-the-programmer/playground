package main

import (
	"fmt"
	"io"
)

type ArrayWriter struct {
	w     io.Writer
	hex   bool
	count int
}

func (aw *ArrayWriter) Write(bs []byte) (n int, err error) {
	if aw.hex {
		return aw.writeHex(bs)
	}

	return aw.writeChar(bs)
}

func (aw ArrayWriter) writeChar(bs []byte) (n int, err error) {
	for _, b := range bs {
		if b == 0 {
			aw.w.Write([]byte("0"))
			aw.w.Write([]byte(",\n"))
		} else {
			aw.w.Write([]byte("'"))
			if b == '\'' {
				aw.w.Write([]byte("\\'"))
			} else if b == '\n' {
				aw.w.Write([]byte("\\n"))
			} else {
				aw.w.Write([]byte{byte(b)})
			}
			aw.w.Write([]byte("', "))
		}
	}

	return len(bs), nil
}

func (aw *ArrayWriter) writeHex(bs []byte) (n int, err error) {
	for _, b := range bs {
		fmt.Fprintf(aw.w, "0x%02x,", b)
		if aw.count%10 == 9 {
			aw.w.Write([]byte("\n"))
		} else {
			aw.w.Write([]byte(" "))
		}
		aw.count++
	}

	return len(bs), nil
}
