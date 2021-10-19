package writers

import (
	"io"
)

type Enum struct {
	w io.WriteCloser
}

func NewEnum(w io.Writer, header, footer WriteCb) io.WriteCloser {
	ident := NewIdent(w, "    ")
	headerFooter := NewHeaderFooter(ident, header, footer)

	return &Enum{
		w: headerFooter,
	}
}

func (w *Enum) Write(p []byte) (n int, err error) {
	return w.w.Write(p)
}

func (w *Enum) Close() error {
	return w.w.Close()
}
