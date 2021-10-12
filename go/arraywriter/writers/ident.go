package writers

import (
	"io"
	"bytes"
)

type Ident struct {
	w      io.Writer
	char   byte
	repeat int
	nl     bool
}

func NewIdent(w io.Writer, char byte, repeat int, nl bool) io.Writer {
	return &Ident{
		w:      w,
		char:   char,
		repeat: repeat,
		nl:     nl,
	}
}

func (w *Ident) Write(p []byte) (n int, err error) {
	b := p[:]

	for {
		len := len(b)
		if len == 0 {
			break
		}

		if w.nl {
			w.writeIdent()
			w.nl = false
		}

		i := bytes.IndexByte(b, '\n')
		if i == -1 {
			w.w.Write(b)
			break
		} else {
			w.nl = true
			if i == len {
				w.w.Write(b)
				break
			} else {
				w.w.Write(b[:i+1])
				b = b[i+1:]
			}
		}
	}

	return len(p), err
}

func (w *Ident) writeIdent() {
	for i := 0; i < w.repeat; i++ {
		w.w.Write([]byte{w.char})
	}
}
