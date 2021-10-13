package writers

import (
	"io"
)

type WriteCb func(w io.Writer)

type HeaderFooter struct {
	w      io.Writer
	count  int
	header WriteCb
	footer WriteCb
}

func NewHeaderFooter(w io.Writer, header WriteCb, footer WriteCb) io.WriteCloser {
	return &HeaderFooter{
		w:      w,
		header: header,
		footer: footer,
	}
}

func (w *HeaderFooter) Write(p []byte) (n int, err error) {
	if w.count == 0 && w.header != nil {
		w.header(w.w)
	}

	w.w.Write(p)

	len := len(p)
	w.count += len

	return len, nil
}

func (w *HeaderFooter) Close() error {
	if w.footer != nil {
		w.footer(w.w)
	}

	return nil
}
