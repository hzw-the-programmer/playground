package writers

import (
	"encoding/hex"
	"io"
)

type Hex struct {
	w io.Writer
	cols int
	count int
}

func NewHex(w io.Writer, cols int) io.Writer {
	return &Hex{
		w: w,
		cols: cols,
	}
}

func (w *Hex) Write(p []byte) (n int, err error) {
	for _, b := range p {
		var dst [2]byte

		hex.Encode(dst[:], []byte{b})

		if w.count != 0 {
			if w.cols == 0 || w.count % w.cols != 0 {
				w.w.Write([]byte{' '})	
			}	
		}
		
		w.w.Write([]byte("0x"))
		w.w.Write(dst[:])
		w.w.Write([]byte{','})

		w.count++
	}
	
	return len(p), nil
}
