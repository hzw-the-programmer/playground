package writers

import (
	"encoding/hex"
	"io"
)

type Hex struct {
	w io.Writer
}

func NewHex(w io.Writer) io.Writer {
	return &Hex{
		w: w,
	}
}

func (w *Hex) Write(p []byte) (n int, err error) {
	var dst [2]byte

	for _, b := range p {
		hex.Encode(dst[:], []byte{b})

		w.w.Write([]byte("0x"))
		w.w.Write(dst[:])
		w.w.Write([]byte(", "))
	}

	return len(p), nil
}
