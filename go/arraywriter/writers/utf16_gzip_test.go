package writers

import (
	"bytes"
	"io"
	"testing"
	// "fmt"
	// "compress/gzip"
	// "golang.org/x/text/encoding/unicode"
	// "golang.org/x/text/transform"
)

func TestUtf16Gip(t *testing.T) {
	header := `static const unsigned char en[] =
{
`
	footer := `
};`

	strs := []string{
		"hello world!",
		"I'm Zhiwen He",
		"a happy coder",
	}

	want := `static const unsigned char en[] =
{
    0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 
    0x04, 0xc0, 0xc1, 0x09, 0xc3, 0x20, 0x14, 0x00, 0xd0, 0x37, 
    0xca, 0xef, 0xa9, 0xab, 0xb4, 0x2b, 0xf4, 0x26, 0xf5, 0x83, 
    0x01, 0x13, 0xc5, 0x8b, 0x64, 0xfb, 0xbc, 0x26, 0x75, 0xdd, 
    0x10, 0xb6, 0x61, 0xe9, 0xaa, 0x17, 0xbe, 0xde, 0x4e, 0xe1, 
    0xa7, 0x39, 0x6c, 0xe9, 0x12, 0x3e, 0x12, 0x45, 0x68, 0x8a, 
    0x69, 0xba, 0x85, 0xbf, 0xa1, 0x4a, 0x0b, 0x4f, 0x00, 0x00, 
    0x00, 0xff, 0xff, 0x8a, 0x93, 0x00, 0xfb, 0x52, 0x00, 0x00, 
    0x00, 
};`

	var buf bytes.Buffer

	headerCb := func(w io.Writer) {
		w.Write([]byte(header))
	}

	footerCb := func(w io.Writer) {
		w.Write([]byte(footer))
	}

	w := NewUtf16Gzip(&buf, headerCb, footerCb)

	for _, str := range strs {
		w.Write([]byte(str))
		w.Write([]byte{0})
	}

	w.Close()

	got := buf.String()

	// buf.Reset()
	// gzip := gzip.NewWriter(&buf)
	// utf16 := transform.NewWriter(gzip, unicode.UTF16(unicode.LittleEndian, unicode.IgnoreBOM).NewEncoder())
	// for _, str := range strs {
	// 	utf16.Write([]byte(str))
	// 	utf16.Write([]byte{0})
	// }
	// utf16.Close()
	// gzip.Close()
	// fmt.Printf("%x", buf.Bytes())

	if got != want {
		t.Errorf("\ngot:\n%s\nwant:\n%s", got, want)
	}
}
