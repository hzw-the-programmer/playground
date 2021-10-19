package writers

import (
	"compress/gzip"
	"io"

	"golang.org/x/text/encoding/unicode"
	"golang.org/x/text/transform"
)

type Lang struct {
	w io.Writer
	utf16        io.WriteCloser
	gzip         io.WriteCloser
	headerFooter io.WriteCloser
}

func NewLangExt(w io.Writer, header, footer WriteCb, isUtf16, isBinary bool) io.WriteCloser {
	var wrapped io.Writer
	var utf16 io.WriteCloser
	var gz io.WriteCloser
	var headerFooter io.WriteCloser

	if isBinary {
		gz = gzip.NewWriter(w)
		wrapped = gz
		if isUtf16 {
			utf16 = transform.NewWriter(gz, unicode.UTF16(unicode.LittleEndian, unicode.IgnoreBOM).NewEncoder())
			wrapped = utf16
		}
	} else {
		ident := NewIdent(w, "    ")
		headerFooter = NewHeaderFooter(ident, header, footer)
		width := NewWidth(headerFooter, 10*6)
		hex := NewHex(width, true)
		gz = gzip.NewWriter(hex)
		wrapped = gz
		if isUtf16 {
			utf16 = transform.NewWriter(gz, unicode.UTF16(unicode.LittleEndian, unicode.IgnoreBOM).NewEncoder())
			wrapped = utf16
		}	
	}

	return &Lang{
		w: wrapped,
		utf16:        utf16,
		gzip:         gz,
		headerFooter: headerFooter,
	}
}

func NewLangUtf16(w io.Writer, header, footer WriteCb) io.WriteCloser {
	return NewLangExt(w, header, footer, true, false)
}

func NewLangUtf16Binary(w io.Writer, header, footer WriteCb) io.WriteCloser {
	return NewLangExt(w, header, footer, true, true)
}

func NewLang(w io.Writer, header, footer WriteCb) io.WriteCloser {
	return NewLangExt(w, header, footer, false, false)
}

func NewLangBinary(w io.Writer, header, footer WriteCb) io.WriteCloser {
	return NewLangExt(w, header, footer, false, true)
}

func (w *Lang) Write(data []byte) (n int, err error) {
	return w.w.Write(data)
}

func (w *Lang) Close() error {
	if w.utf16 != nil {
		w.utf16.Close()
	}

	if w.gzip != nil {
		w.gzip.Close()
	}

	if w.headerFooter != nil {
		w.headerFooter.Close()
	}

	return nil
}
