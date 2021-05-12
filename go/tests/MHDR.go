package main

import (
	"fmt"
	"encoding/hex"
)

type MType byte
type Major byte

type MHDR struct {
	MType MType
	Major Major
}

func (h MHDR) MarshalBinary() ([]byte, error) {
	return []byte{byte(h.MType) << 5 | byte(h.Major)}, nil
}

func (h *MHDR) UnmarshalBinary(b []byte) error {
	h.MType = MType(b[0] >> 5)
	h.Major = Major(b[0] & 0x03)
	return nil
}

func main() {
	m := MHDR{5, 2}
	b, _ := m.MarshalBinary()
	fmt.Println(hex.EncodeToString(b))

	var m1 MHDR
	m1.UnmarshalBinary([]byte{0xa2})
	fmt.Println(m1.MType, m1.Major)

	var m2 MHDR
	m2.UnmarshalBinary([]byte{0xff})
	fmt.Println(m2.MType, m2.Major)
}
