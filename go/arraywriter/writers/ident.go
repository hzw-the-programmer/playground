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

	j := 0

	for i, b := range p {
		if b == '}' {
			w.repeat--
		}

		if w.nl {
			w.nl = false
			if b != '\n' {
				w.w.Write(p[j:i])
				j = i
				w.writeIdent()
			}
		}

		if b == '\n' {
			w.nl = true
		} else if b == '{' {
			w.repeat++
		}
	}

	w.w.Write(p[j:])

	return
}

func (w *Ident) writeIdent() {
	for i := 0; i < w.repeat; i++ {
		w.w.Write(w.ident)
	}
}
