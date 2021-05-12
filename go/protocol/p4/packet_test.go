/**
 * Author: Zhiwen He <18676797056@163.com>
 */
package p4

import (
	"encoding/hex"
	"fmt"
	"testing"
	"time"

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
				hex: "5434" +
					"0031" +
					"00001258390438c1" +
					"12345678" +
					"120117170b0b" +
					"45" +
					"12345678" +
					"02" +
					"01000b023333214233332542" +
					"02000d009a999e429a99a042" +
					"e7",
				packet: Packet{
					Header: &Header{
						From: 0x54,
						Ver:  0x34,
						Len:  49,
						Sn:   20170123000001,
						Seq:  0x12345678,
						Time: time.Date(2018, 01, 23, 23, 11, 11, 0, time.Local),
					},
					Body: &HeartBeatWithData{
						Seq: 0x78563412,
						Channels: []Channel{
							{
								Slot:    1,
								Port:    0,
								Type:    ChanTypeTemp,
								Status:  0x02,
								Data:    40.3,
								AvgData: 41.3,
							},
							{
								Slot:    2,
								Port:    0,
								Type:    ChanTypeHumi,
								Status:  0x00,
								Data:    79.3,
								AvgData: 80.3,
							},
						},
					},
				},
				err: nil,
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
