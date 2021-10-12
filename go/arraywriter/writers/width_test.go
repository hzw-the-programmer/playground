package writers

import (
	"testing"
	"bytes"
)

func TestWidth(t *testing.T) {
	src := "hello world!"
	want := "    0x68, 0x65, 0x6c, \n    0x6c, 0x6f, 0x20, \n    0x77, 0x6f, 0x72, \n    0x6c, 0x64, 0x21, \n"
	
	var buf bytes.Buffer
	h := NewHex(&buf)
	w := NewWidth(&buf, h, 3)

	w.Write([]byte(src))
	dst := buf.String()

	if dst != want {
		t.Errorf("%s != %s", dst, want)
	}
}