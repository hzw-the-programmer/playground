package lorawan

import (
	"encoding/binary"
	"errors"
)

type ChMask [16]bool

func (cm ChMask) MarshalBinary() ([]byte, error) {
	var n uint16
	for i := 0; i < 16; i++ {
		if cm[i] {
			n |= 1 << uint(i)
		}
	}

	b := make([]byte, 2)
	binary.LittleEndian.PutUint16(b, n)

	return b, nil
}

func (cm *ChMask) UnmarshalBinary(b []byte) error {
	if len(b) != 2 {
		return errors.New("ChMask.UnmarshalBinary expects 2 bytes")
	}

	n := binary.LittleEndian.Uint16(b)
	for i := 0; i < 16; i++ {
		if n&(1<<uint(i)) != 0 {
			cm[i] = true
		}
	}

	return nil
}
