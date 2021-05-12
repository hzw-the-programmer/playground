package lorawan

import (
	"encoding/binary"
	"encoding/hex"
)

type NetID [3]byte

func (n NetID) Type() int {
	return int(n[0] >> 5)
}

func (n NetID) ID() []byte {
	switch n.Type() {
	case 0, 1:
		return n.getID(6)
	case 2:
		return n.getID(9)
	case 3, 4, 5, 6, 7:
		return n.getID(21)
	default:
		return nil
	}
}

func (n NetID) getID(bits int) []byte {
	b := make([]byte, 4)
	copy(b[1:], n[:])
	t := binary.BigEndian.Uint32(b)
	t <<= 32 - uint(bits)
	t >>= 32 - uint(bits)
	binary.BigEndian.PutUint32(b, t)
	l := bits / 8
	if bits%8 != 0 {
		l++
	}
	return b[len(b)-l:]
}

func (n NetID) MarshalBinary() ([]byte, error) {
	data := make([]byte, len(n))
	for i, v := range n {
		data[len(n)-1-i] = v
	}
	return data, nil
}

func (n *NetID) UnmarshalBinary(data []byte) error {
	for i, v := range data {
		n[len(n)-1-i] = v
	}
	return nil
}

func (n NetID) MarshalText() ([]byte, error) {
	return []byte(n.String()), nil
}

func (n *NetID) UnmarshalText(data []byte) error {
	b, err := hex.DecodeString(string(data))
	if err != nil {
		return err
	}
	copy(n[:], b)
	return nil
}

func (n NetID) String() string {
	return hex.EncodeToString(n[:])
}
