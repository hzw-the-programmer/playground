package writers

import (
	"io"
)

type Width struct {
	w     io.Writer
	cols  int
	count int
}

func NewWidth(w io.Writer, cols int) io.WriteCloser {
	return &Width{
		w:    w,
		cols: cols,
	}
}

func (w *Width) Write(p []byte) (n int, err error) {
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
		}

		if len > w.cols-col {
			len = w.cols - col
		}

		w.w.Write(b[:len])
		b = b[len:]
		w.count += len
	}

	return len(p), nil
}

func (w *Width) Close() error {
	if c, ok := w.w.(io.Closer); ok {
		c.Close()
	}

	return nil
}
