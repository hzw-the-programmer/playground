package writers

import (
	"compress/gzip"
	"io"

	"golang.org/x/text/encoding/unicode"
	"golang.org/x/text/transform"
)

type Lang struct {
	utf16        io.WriteCloser
	gzip         io.WriteCloser
	headerFooter io.WriteCloser
}

func NewLang(w io.Writer, header WriteCb, footer WriteCb) io.WriteCloser {
	ident := NewIdent(w, "    ")
	headerFooter := NewHeaderFooter(ident, header, footer)
	width := NewWidth(headerFooter, 10*6)
	hex := NewHex(width, true)
	gzip := gzip.NewWriter(hex)
	utf16 := transform.NewWriter(gzip, unicode.UTF16(unicode.LittleEndian, unicode.IgnoreBOM).NewEncoder())

	return &Lang{
		utf16:        utf16,
		gzip:         gzip,
		headerFooter: headerFooter,
	}
}

func (w *Lang) Write(data []byte) (n int, err error) {
	return w.utf16.Write(data)
}

func (w *Lang) Close() error {
	w.utf16.Close()
	w.gzip.Close()
	w.headerFooter.Close()

	return nil
}
