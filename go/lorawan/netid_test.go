package lorawan

import (
	"fmt"
	"testing"

	. "github.com/smartystreets/goconvey/convey"
)

func TestNetID(t *testing.T) {
	Convey("Given a set of tests", t, func() {
		tests := []struct {
			Name   string
			NetID  NetID
			Type   int
			ID     []byte
			Bytes  []byte
			String string
		}{
			{
				Name:   "NetID type 0",
				NetID:  NetID{0x00, 0x00, 0x6d}, // 0000 0000 0000 0000 0110 1101
				Type:   0,
				ID:     []byte{0x2d},
				Bytes:  []byte{0x6d, 0x00, 0x00},
				String: "00006d",
			},
			{
				Name:   "NetID type 1",
				NetID:  NetID{0x20, 0x00, 0x6d}, // 0010 0000 0000 0000 0110 1101
				Type:   1,
				ID:     []byte{0x2d},
				Bytes:  []byte{0x6d, 0x00, 0x20},
				String: "20006d",
			},
			{
				Name:   "NetID type 2",
				NetID:  NetID{0x40, 0x03, 0x6d}, // 0100 0000 0000 0011 0110 1101
				Type:   2,
				ID:     []byte{0x01, 0x6d},
				Bytes:  []byte{0x6d, 0x03, 0x40},
				String: "40036d",
			},
			{
				Name:   "NetID type 3",
				NetID:  NetID{0x76, 0xdb, 0x6d}, // 0111 0110 1101 1011 0110 1101
				Type:   3,
				ID:     []byte{0x16, 0xdb, 0x6d},
				Bytes:  []byte{0x6d, 0xdb, 0x76},
				String: "76db6d",
			},
			{
				Name:   "NetID type 4",
				NetID:  NetID{0x96, 0xdb, 0x6d}, // 1001 0110 1101 1011 0110 1101
				Type:   4,
				ID:     []byte{0x16, 0xdb, 0x6d},
				Bytes:  []byte{0x6d, 0xdb, 0x96},
				String: "96db6d",
			},
			{
				Name:   "NetID type 5",
				NetID:  NetID{0xb6, 0xdb, 0x6d}, // 1011 0110 1101 1011 0110 1101
				Type:   5,
				ID:     []byte{0x16, 0xdb, 0x6d},
				Bytes:  []byte{0x6d, 0xdb, 0xb6},
				String: "b6db6d",
			},
			{
				Name:   "NetID type 6",
				NetID:  NetID{0xd6, 0xdb, 0x6d}, // 1101 0110 1101 1011 0110 1101
				Type:   6,
				ID:     []byte{0x16, 0xdb, 0x6d},
				Bytes:  []byte{0x6d, 0xdb, 0xd6},
				String: "d6db6d",
			},
			{
				Name:   "NetID type 7",
				NetID:  NetID{0xf6, 0xdb, 0x6d}, // 1111 0110 1101 1011 0110 1101
				Type:   7,
				ID:     []byte{0x16, 0xdb, 0x6d},
				Bytes:  []byte{0x6d, 0xdb, 0xf6},
				String: "f6db6d",
			},
		}

		for i, test := range tests {
			Convey(fmt.Sprintf("Testing: %s [%d]", test.Name, i), func() {
				So(test.NetID.Type(), ShouldEqual, test.Type)
				So(test.NetID.ID(), ShouldResemble, test.ID)

				b, err := test.NetID.MarshalBinary()
				So(err, ShouldBeNil)
				So(b, ShouldResemble, test.Bytes)

				var netID NetID
				So(netID.UnmarshalBinary(test.Bytes), ShouldBeNil)
				So(netID, ShouldEqual, test.NetID)

				b, err = test.NetID.MarshalText()
				So(err, ShouldBeNil)
				So(string(b), ShouldEqual, test.String)
				//So(b, ShouldResemble, []byte(test.String))

				So(netID.UnmarshalText([]byte(test.String)), ShouldBeNil)
				So(netID, ShouldEqual, test.NetID)
			})
		}
	})
}
