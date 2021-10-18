package writers

import (
	"io"
)

type Ident struct {
	w      io.Writer
	ident  []byte
	repeat int
	nl     bool
}

func NewIdent(w io.Writer, ident string) io.Writer {
	return &Ident{
		w:     w,
		ident: []byte(ident),
	}
}

func (w *Ident) Write(p []byte) (n int, err error) {
	n = len(p)

	for _, b := range p {
		if b == '}' {
			w.repeat--
		}

		if w.nl {
			if b != '\n' {
				w.writeIdent()
			}
			w.nl = false
		}

		if b == '{' {
			w.repeat++
		} else if b == '\n' {
			w.nl = true
		}

		w.w.Write([]byte{b})
	}

	return
}

func (w *Ident) writeIdent() {
	for i := 0; i < w.repeat; i++ {
		w.w.Write(w.ident)
	}
}
