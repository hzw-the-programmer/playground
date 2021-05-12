/**
 * Author: Zhiwen He <18676797056@163.com>
 */
package idas

import (
	"encoding/hex"
	"fmt"
	"testing"

	. "github.com/smartystreets/goconvey/convey"
)

func TestHeartBeatWithData(t *testing.T) {
	Convey("Given a set of tests", t, func() {
		tests := []struct {
			hex  string
			body HeartBeatWithData
			err  error
		}{
			{
				hex: "45" +
					"12345678" +
					"02" +
					"01000b023333214233332542" +
					"02000d009a999e429a99a042",
				body: HeartBeatWithData{
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
				err: nil,
			},
			{
				hex: "46" +
					"12345678" +
					"02" +
					"01000b023333214233332542" +
					"02000d009a999e429a99a042",
				body: HeartBeatWithData{},
				err:  ErrNotHeartBeatWithData,
			},
			{
				hex: "45" +
					"12345678",
				body: HeartBeatWithData{},
				err:  ErrHeartBeatWithDataAtLeast6Bytes,
			},
			{
				hex: "45" +
					"12345678" +
					"02" +
					"01000b023333214233332542" +
					"02000d009a999e429a99a0",
				body: HeartBeatWithData{},
				err:  ErrHeartBeatWithDataLen,
			},
			{
				hex: "45" +
					"12345678" +
					"00",
				body: HeartBeatWithData{
					Seq: 0x78563412,
				},
				err: nil,
			},
			{
				hex: "45" +
					"12345678" +
					"03" +
					"01000b023333214233332542" +
					"02000b023333214233332542" +
					"03000d009a999e429a99a042",
				body: HeartBeatWithData{
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
							Type:    ChanTypeTemp,
							Status:  0x02,
							Data:    40.3,
							AvgData: 41.3,
						},
						{
							Slot:    3,
							Port:    0,
							Type:    ChanTypeHumi,
							Status:  0x00,
							Data:    79.3,
							AvgData: 80.3,
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

				var body HeartBeatWithData
				err = body.Unmarshal(bs)
				So(err, ShouldResemble, test.err)
				So(body, ShouldResemble, test.body)

				if err == nil {
					bbs, err := test.body.Marshal()
					So(err, ShouldBeNil)
					So(bbs, ShouldResemble, bs)
				}
			})
		}
	})
}

func TestStatusData(t *testing.T) {
	Convey("Given a set of tests", t, func() {
		tests := []struct {
			hex  string
			body StatusData
			err  error
		}{
			{
				hex: "40" +
					"03" +
					"1008020000b841" +
					"200c0000003041" +
					"30095000004040",
				body: StatusData{
					Channels: []Channel{
						{
							Slot:   1,
							Port:   0,
							Type:   ChanTypeGndH,
							Status: 0x02,
							Data:   23,
						},
						{
							Slot:   2,
							Port:   0,
							Type:   ChanTypeGndL,
							Status: 0x00,
							Data:   11,
						},
						{
							Slot:   3,
							Port:   0,
							Type:   ChanTypeWS,
							Status: 0x50,
							Data:   3,
						},
					},
				},
				err: nil,
			},
			{
				hex: "41" +
					"03" +
					"1008020000b841" +
					"200c0000003041" +
					"30095000004040",
				body: StatusData{},
				err:  ErrNotStatusData,
			},
			{
				hex:  "40",
				body: StatusData{},
				err:  ErrStatusDataAtLeast2Bytes,
			},
			{
				hex: "40" +
					"00",
				body: StatusData{},
				err:  nil,
			},
			{
				hex: "40" +
					"00" +
					"1008020000b841",
				body: StatusData{},
				err:  ErrStatusDataLen,
			},
			{
				hex: "40" +
					"01" +
					"1008020000b8",
				body: StatusData{},
				err:  ErrStatusDataLen,
			},
			{
				hex: "40" +
					"02" +
					"1008020000b841" +
					"200c000000304112",
				body: StatusData{},
				err:  ErrStatusDataLen,
			},
			{
				hex: "40" +
					"03" +
					"1008020000b841" +
					"200c0000003041",
				body: StatusData{},
				err:  ErrStatusDataLen,
			},
		}

		for i, test := range tests {
			Convey(fmt.Sprintf("Testing: %d", i), func() {
				bs, err := hex.DecodeString(test.hex)
				if err != nil {
					t.Fatal(err)
				}

				var body StatusData
				err = body.Unmarshal(bs)
				So(err, ShouldResemble, test.err)
				So(body, ShouldResemble, test.body)

				if err == nil {
					bbs, err := test.body.Marshal()
					So(err, ShouldBeNil)
					So(bbs, ShouldResemble, bs)
				}
			})
		}
	})
}
