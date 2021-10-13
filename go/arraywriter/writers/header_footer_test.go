package writers

import (
	"bytes"
	// "fmt"
	"io"
	"testing"
)

func TestHeaderFooter(t *testing.T) {
	var buf bytes.Buffer

	wrapped := NewIdent(&buf, "    ")

	header := func(w io.Writer) {
		c := `static const unsigned char en[] =
{
`
		w.Write([]byte(c))
	}
	footer := func(w io.Writer) {
		c := `
};
`
w.Write([]byte(c))
	}

	wrapped = NewHeaderFooter(wrapped, header, footer)
	wrapped = NewWidth(wrapped, 10*6)
	wrapped = NewHex(wrapped, true)

out:
	for i, j := 0, 1; ; j++ {
		for k := 0; k < j; k++ {
			wrapped.Write([]byte{byte(i)})
			if i++; i == 21 {
				break out
			}
		}
	}

	wrapped.Close()

	got := buf.String()

	want := `static const unsigned char en[] =
{
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 
    0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 
    0x14, 
};
`

	if got != want {
		t.Errorf("\ngot:\n%s\nwant:\n%s", got, want)
	}
}
