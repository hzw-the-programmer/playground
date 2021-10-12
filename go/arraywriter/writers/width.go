package writers

import (
	"io"
)

type width struct {
	w     io.Writer
	ww    io.Writer
	cols  int
	count int
}

func NewWidth(w io.Writer, ww io.Writer, cols int) io.Writer {
	return &width{
		w:    w,
		ww:   ww,
		cols: cols,
	}
}

func (w *width) Write(p []byte) (n int, err error) {
	b := p[:]

	for {
		len := len(b)
		if len == 0 {
			break
		}

		col := w.count % w.cols
		if col == 0 {
			if w.count != 0 {
				w.w.Write([]byte{'\n'})
			}
			w.w.Write([]byte("    "))
		}

		if len > w.cols-col {
			len = w.cols - col
		}

		w.ww.Write(b[:len])
		b = b[len:]
		w.count += len
	}

	return len(p), nil
}
