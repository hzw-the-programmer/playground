package lorawan

import (
	"encoding/binary"
	"encoding/hex"
)

type DevAddr [4]byte

func (a DevAddr) NetIDType() int {
	for i := 7; i >= 0; i-- {
		if a[0]&(1<<byte(i)) == 0 {
			return 7 - i
		}
	}
	panic("NetIDType bug")
}

func (a DevAddr) NwkID() []byte {
	switch a.NetIDType() {
	case 0:
		return a.getNwkID(1, 6)
	case 1:
		return a.getNwkID(2, 6)
	case 2:
		return a.getNwkID(3, 9)
	case 3:
		return a.getNwkID(4, 10)
	case 4:
		return a.getNwkID(5, 11)
	case 5:
		return a.getNwkID(6, 13)
	case 6:
		return a.getNwkID(7, 15)
	case 7:
		return a.getNwkID(8, 17)
	default:
		return nil
	}
}

func (a DevAddr) getNwkID(prefixLength, nwkIDBits int) []byte {
	temp := binary.BigEndian.Uint32(a[:])
	temp <<= uint32(prefixLength)
	temp >>= 32 - uint32(nwkIDBits)
	bin := make([]byte, len(a))
	binary.BigEndian.PutUint32(bin, temp)
	l := nwkIDBits / 8
	if nwkIDBits%8 != 0 {
		l++
	}
	return bin[len(a)-l:]
}

func (a DevAddr) MarshalBinary() ([]byte, error) {
	bin := make([]byte, len(a))
	for i, v := range a {
		bin[len(a)-1-i] = v
	}
	return bin, nil
}

func (a *DevAddr) UnmarshalBinary(bin []byte) error {
	for i, v := range bin {
		a[len(a)-1-i] = v
	}
	return nil
}

func (a DevAddr) MarshalText() ([]byte, error) {
	return []byte(a.String()), nil
}

func (a *DevAddr) UnmarshalText(text []byte) error {
	bin, err := hex.DecodeString(string(text))
	if err != nil {
		return err
	}
	copy(a[:], bin)
	return nil
}

func (a DevAddr) String() string {
	return hex.EncodeToString(a[:])
}
