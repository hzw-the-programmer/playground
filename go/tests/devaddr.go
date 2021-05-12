package main

import (
	"fmt"
	"encoding/binary"
)

type DevAddr [4]byte

func (a DevAddr) NetIDType() int {
	for i := 7; i >= 0; i-- {
		if a[0] & (1 << byte(i)) == 0 {
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
	out := make([]byte, 4)
	binary.BigEndian.PutUint32(out, temp)
	l := nwkIDBits / 8
	if nwkIDBits % 8 != 0 {
		l += 1
	}
	return out[len(out) - l:]
}

func main() {
	d := DevAddr{0x5b, 0xff, 0xff, 0xff} // 0101 1011
	fmt.Println(d.NetIDType()) // 0
	fmt.Println(d.NwkID()) // 0x2d
	d = DevAddr{0xad, 0xff, 0xff, 0xff} // 1010 1101
	fmt.Println(d.NetIDType()) // 1
	fmt.Println(d.NwkID()) // 0x2d
	d = DevAddr{0xd6, 0xdf, 0xff, 0xff} // 1101 0110 1101 1111
	fmt.Println(d.NetIDType()) // 2
	fmt.Println(d.NwkID()) // 0x016d
	d = DevAddr{0xeb, 0x6f, 0xff, 0xff} // 1110 1011 0110 1111
	fmt.Println(d.NetIDType()) // 3
	fmt.Println(d.NwkID()) // 0x02db
	d = DevAddr{0xf5, 0xb6, 0xff, 0xff} // 1111 0101 1011 0110
	fmt.Println(d.NetIDType()) // 4
	fmt.Println(d.NwkID()) // 0x05b6
	d = DevAddr{0xfa, 0xdb, 0x7f, 0xff} // 1111 1010 1101 1011 0111 1111
	fmt.Println(d.NetIDType()) // 5
	fmt.Println(d.NwkID()) // 0x16db
	d = DevAddr{0xfd, 0x6d, 0xb7, 0xff} // 1111 1101 0110 1101 1011 0111
	fmt.Println(d.NetIDType()) // 6
	fmt.Println(d.NwkID()) // 0x5b6d
	d = DevAddr{0xfe, 0xb6, 0xdb, 0x7f} // 1111 1110 1011 0110 1101 1011 0111 1111
	fmt.Println(d.NetIDType()) // 7
	fmt.Println(d.NwkID()) // 0x016db6
}
