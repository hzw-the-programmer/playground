package writers

import (
	"encoding/hex"
	"io"
)

type Hex struct {
	w     io.Writer
	space bool
}

func NewHex(w io.Writer, space bool) io.WriteCloser {
	return &Hex{
		w:     w,
		space: space,
	}
}

func (w *Hex) Write(p []byte) (n int, err error) {
	for _, b := range p {
		var dst [2]byte

		hex.Encode(dst[:], []byte{b})

		w.w.Write([]byte("0x"))
		w.w.Write(dst[:])
		w.w.Write([]byte{','})

		if w.space {
			w.w.Write([]byte{' '})
		}
	}

	return len(p), nil
}

func (w *Hex) Close() error {
	if c, ok := w.w.(io.Closer); ok {
		c.Close()
	}

	return nil
}
