package writers

import (
	"testing"
	"bytes"
)

func TestHeaderFooter(t *testing.T) {
	cols := 7
	header := "static const unsigned char en = {\n"
	content := "hello world!"
	footer := "\n};"
	want := `static const unsigned char en = {
    0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77,
    0x6f, 0x72, 0x6c, 0x64, 0x21,
};`
	
	var buf bytes.Buffer
	hex := NewHex(&buf, cols)
	width := NewWidth(&buf, hex, cols)
	w := NewHeaderFooter(&buf, width, header, footer)
	
	w.Write([]byte(content))
	w.Close()

	got := buf.String()

	if got != want {
		t.Errorf("\ngot:\n%s\nwant:\n%s", got, want)
	}
}