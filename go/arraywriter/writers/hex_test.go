package writers

import (
	"bytes"
	"io"
	"strings"
	"testing"
)

func TestHex(t *testing.T) {
	src := "hello world!"
	want := "0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21,"

	reader := strings.NewReader(src)
	var buf bytes.Buffer
	writer := NewHex(&buf, 0)
	io.Copy(writer, reader)
	dst := buf.String()

	if dst != want {
		t.Errorf("%s != %s", dst, want)
	}
}
