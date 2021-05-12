/**
 * Author: Zhiwen He <18676797056@163.com>
 */
package oulang

import (
	"encoding/hex"
	"fmt"
	"testing"

	. "github.com/smartystreets/goconvey/convey"
)

func TestPacket(t *testing.T) {
	Convey("Given a set of tests", t, func() {
		tests := []struct {
			hex    string
			packet Packet
			err    error
		}{
			{
				hex: "ff1a01aa",
				packet: Packet{
					Battery:     0xff,
					Temperature: 0x1a,
					Type:        PTHeartBeat,
				},
				err: nil,
			},
			{
				hex: "ff1a4201094133332142aa",
				packet: Packet{
					Battery:     0xff,
					Temperature: 0x1a,
					Type:        PTStatus,
					Channel:     1,
					CType:       CTGNDH,
					Status:      0x41,
					Data:        40.3,
				},
				err: nil,
			},
			{
				hex:    "ff1a01",
				packet: Packet{},
				err:    ErrAtLeast4Bytes,
			},
			{
				hex:    "ff1a01ab",
				packet: Packet{},
				err:    ErrWrongTerminator,
			},
		}

		for i, test := range tests {
			Convey(fmt.Sprintf("Testing: %d", i), func() {
				bs, err := hex.DecodeString(test.hex)
				if err != nil {
					t.Fatal(err)
				}

				var packet Packet
				err = packet.Unmarshal(bs)
				So(err, ShouldResemble, test.err)
				So(packet, ShouldResemble, test.packet)

				if err == nil {
					pbs, err := test.packet.Marshal()
					So(err, ShouldBeNil)
					So(pbs, ShouldResemble, bs)
				}
			})
		}
	})
}
