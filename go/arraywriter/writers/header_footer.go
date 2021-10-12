package writers

import (
	"io"
)

type HeaderFooter struct {
	w      io.Writer
	ww     io.Writer
	count  int
	header []byte
	footer []byte
}

func NewHeaderFooter(w io.Writer, ww io.Writer, header string, footer string) io.WriteCloser {
	return &HeaderFooter{
		w:      w,
		ww:     ww,
		header: []byte(header),
		footer: []byte(footer),
	}
}

func (w *HeaderFooter) Write(p []byte) (n int, err error) {
	if w.count == 0 {
		w.w.Write(w.header)
	}

	w.ww.Write(p)

	len := len(p)
	w.count += len

	return len, nil
}

func (w *HeaderFooter) Close() error {
	w.w.Write(w.footer)

	return nil
}
