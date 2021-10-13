package writers

import (
	"encoding/hex"
	"io"
)

type Hex struct {
	w     io.Writer
}

func NewHex(w io.Writer) io.Writer {
	return &Hex{
		w:    w,
	}
}

func (w *Hex) Write(p []byte) (n int, err error) {
	for _, b := range p {
		var dst [2]byte

		hex.Encode(dst[:], []byte{b})

		w.w.Write([]byte("0x"))
		w.w.Write(dst[:])
		w.w.Write([]byte{','})
	}

	return len(p), nil
}
