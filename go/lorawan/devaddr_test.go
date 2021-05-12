package lorawan

import (
	"fmt"
	"testing"

	. "github.com/smartystreets/goconvey/convey"
)

func TestDevAddr(t *testing.T) {
	Convey("Given a set of tests", t, func() {
		tests := []struct {
			Name      string
			DevAddr   DevAddr
			NetIDType int
			NwkID     []byte
			Bytes     []byte
			String    string
		}{
			{
				Name:      "NetID type 0",
				DevAddr:   DevAddr{0x5b, 0xff, 0xff, 0xff}, // 0101 1011
				NetIDType: 0,
				NwkID:     []byte{0x2d},
				Bytes:     []byte{0xff, 0xff, 0xff, 0x5b},
				String:    "5bffffff",
			},
			{
				Name:      "NetID type 1",
				DevAddr:   DevAddr{0xad, 0xff, 0xff, 0xff}, // 1010 1101
				NetIDType: 1,
				NwkID:     []byte{0x2d},
				Bytes:     []byte{0xff, 0xff, 0xff, 0xad},
				String:    "adffffff",
			},
			{
				Name:      "NetID type 2",
				DevAddr:   DevAddr{0xd6, 0xdf, 0xff, 0xff}, // 1101 0110 1101 1111
				NetIDType: 2,
				NwkID:     []byte{0x01, 0x6d},
				Bytes:     []byte{0xff, 0xff, 0xdf, 0xd6},
				String:    "d6dfffff",
			},
			{
				Name:      "NetID type 3",
				DevAddr:   DevAddr{0xeb, 0x6f, 0xff, 0xff}, // 1110 1011 0110 1111
				NetIDType: 3,
				NwkID:     []byte{0x02, 0xdb},
				Bytes:     []byte{0xff, 0xff, 0x6f, 0xeb},
				String:    "eb6fffff",
			},
			{
				Name:      "NetID type 4",
				DevAddr:   DevAddr{0xf5, 0xb6, 0xff, 0xff}, // 1111 0101 1011 0110
				NetIDType: 4,
				NwkID:     []byte{0x05, 0xb6},
				Bytes:     []byte{0xff, 0xff, 0xb6, 0xf5},
				String:    "f5b6ffff",
			},
			{
				Name:      "NetID type 5",
				DevAddr:   DevAddr{0xfa, 0xdb, 0x7f, 0xff}, // 1111 1010 1101 1011 0111 1111
				NetIDType: 5,
				NwkID:     []byte{0x16, 0xdb},
				Bytes:     []byte{0xff, 0x7f, 0xdb, 0xfa},
				String:    "fadb7fff",
			},
			{
				Name:      "NetID type 6",
				DevAddr:   DevAddr{0xfd, 0x6d, 0xb7, 0xff}, // 1111 1101 0110 1101 1011 0111
				NetIDType: 6,
				NwkID:     []byte{0x5b, 0x6d},
				Bytes:     []byte{0xff, 0xb7, 0x6d, 0xfd},
				String:    "fd6db7ff",
			},
			{
				Name:      "NetID type 7",
				DevAddr:   DevAddr{0xfe, 0xb6, 0xdb, 0x7f}, // 1111 1110 1011 0110 1101 1011 0111 1111
				NetIDType: 7,
				NwkID:     []byte{0x01, 0x6d, 0xb6},
				Bytes:     []byte{0x7f, 0xdb, 0xb6, 0xfe},
				String:    "feb6db7f",
			},
		}

		for i, test := range tests {
			Convey(fmt.Sprintf("Testing: %s [%d]", test.Name, i), func() {
				So(test.DevAddr.NetIDType(), ShouldEqual, test.NetIDType)
				So(test.DevAddr.NwkID(), ShouldResemble, test.NwkID)

				bin, err := test.DevAddr.MarshalBinary()
				So(err, ShouldBeNil)
				So(bin, ShouldResemble, test.Bytes)

				var devAddr DevAddr
				So(devAddr.UnmarshalBinary(test.Bytes), ShouldBeNil)
				So(devAddr, ShouldResemble, test.DevAddr)

				text, err := test.DevAddr.MarshalText()
				So(err, ShouldBeNil)
				So(string(text), ShouldEqual, test.String)

				So(devAddr.UnmarshalText([]byte(test.String)), ShouldBeNil)
				So(devAddr, ShouldResemble, test.DevAddr)
			})
		}
	})
}
