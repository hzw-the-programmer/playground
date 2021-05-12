/**
 * Author: Zhiwen He <18676797056@163.com>
 */
package cleargrass

import (
	"encoding/hex"
	"fmt"
	"testing"
	"time"

	. "github.com/smartystreets/goconvey/convey"
)

func TestSensorData(t *testing.T) {
	Convey("Given a set of tests", t, func() {
		tests := []struct {
			sd  SensorData
			hex string
			err error
		}{
			{
				SensorData{},
				"2fc29a2766",
				ErrSensorDataNot6Bytes,
			},
			{
				SensorData{26.4, 66.6, 100.86, 78.0},
				"2fc29a27664e",
				nil,
			},
		}

		for i, test := range tests {
			Convey(fmt.Sprintf("Testing: %d", i), func() {
				b, err := hex.DecodeString(test.hex)
				if err != nil {
					t.Fatalf("hex.DecodeString(%s) error: %s", test.hex, err)
				}
				var sd SensorData
				err = sd.Unmarshal(b)
				So(err, ShouldResemble, test.err)
				So(sd, ShouldResemble, test.sd)

				if err == nil {
					br, err := test.sd.Marshal()
					So(err, ShouldBeNil)
					So(br, ShouldResemble, b)
				}
			})
		}
	})
}

func TestRealTimeData(t *testing.T) {
	Convey("Given a set of tests", t, func() {
		tests := []struct {
			Rtd RealTimeData
			Hex string
			Err error
		}{
			{
				Rtd: RealTimeData{},
				Hex: "015c7788b62fc29a27664e312e302e305f303034",
				Err: ErrRealTimeDataNot21Bytes,
			},
			{
				Rtd: RealTimeData{
					Time: time.Date(2019, 2, 28, 15, 07, 34, 0, time.Local),
					SD:   SensorData{26.4, 66.6, 100.86, 78.0},
					Ver:  "1.0.0_0041",
				},
				Hex: "015c7788b62fc29a27664e312e302e305f30303431",
				Err: nil,
			},
		}

		for i, test := range tests {
			Convey(fmt.Sprintf("Testing: %d", i), func() {
				b, err := hex.DecodeString(test.Hex)
				if err != nil {
					t.Fatalf("hex.DecodeString(%s) error: %s", test.Hex, err)
				}
				var rtd RealTimeData
				err = rtd.Unmarshal(b)
				So(err, ShouldResemble, test.Err)
				So(rtd, ShouldResemble, test.Rtd)

				if err == nil {
					db, err := test.Rtd.Marshal()
					So(err, ShouldBeNil)
					So(db, ShouldResemble, b)
				}
			})
		}
	})
}

func TestHistoryData(t *testing.T) {
	Convey("Given a set of tests", t, func() {
		tests := []struct {
			Hd  HistoryData
			Hex string
			Err error
		}{
			{
				Hd:  HistoryData{},
				Hex: "005c7788b600052fc29a2766",
				Err: ErrHistoryDataAtLeast13Bytes,
			},
			{
				Hd: HistoryData{
					Time:     time.Date(2019, 2, 28, 15, 07, 34, 0, time.Local),
					Interval: 5 * time.Second,
					SDs: []SensorData{
						SensorData{26.4, 66.6, 100.86, 78.0},
					},
				},
				Hex: "005c7788b600052fc29a27664e",
				Err: nil,
			},
		}

		for i, test := range tests {
			Convey(fmt.Sprintf("Testing: %d", i), func() {
				b, err := hex.DecodeString(test.Hex)
				if err != nil {
					t.Fatalf("hex.DecodeString(%s) error: %s", test.Hex, err)
				}
				var hd HistoryData
				err = hd.Unmarshal(b)
				So(err, ShouldResemble, test.Err)
				So(hd, ShouldResemble, test.Hd)

				if err == nil {
					db, err := test.Hd.Marshal()
					So(err, ShouldBeNil)
					So(db, ShouldResemble, b)
				}
			})
		}
	})
}

func TestTimeSync(t *testing.T) {
	Convey("Given a set of tests", t, func() {
		tests := []struct {
			Ts  TimeSync
			Hex string
			Err error
		}{
			{
				Ts:  TimeSync{},
				Hex: "",
				Err: nil,
			},
			{
				Ts: TimeSync{
					Time: time.Date(2019, 5, 27, 17, 02, 17, 0, time.Local),
				},
				Hex: "5ceba799",
				Err: nil,
			},
		}

		for i, test := range tests {
			Convey(fmt.Sprintf("Testing: %d", i), func() {
				b, err := hex.DecodeString(test.Hex)
				if err != nil {
					t.Fatalf("hex.DecodeString(%s) error: %s", test.Hex, err)
				}
				var ts TimeSync
				err = ts.Unmarshal(b)
				So(err, ShouldResemble, test.Err)
				So(ts, ShouldResemble, test.Ts)

				if err == nil {
					db, err := test.Ts.Marshal()
					So(err, ShouldBeNil)
					So(db, ShouldResemble, b)
				}
			})
		}
	})
}

func TestPacket(t *testing.T) {
	Convey("Given a set of tests", t, func() {
		tests := []struct {
			Pkt Packet
			Hex string
			Err error
		}{
			{
				Pkt: Packet{
					Addr: Addr,
					Cmd:  DataUpCmd,
					Len:  0x15,
					Data: &RealTimeData{
						Time: time.Date(2019, 2, 28, 15, 07, 34, 0, time.Local),
						SD:   SensorData{26.4, 66.6, 100.86, 78.0},
						Ver:  "1.0.0_0041",
					},
					Crc: 0xc65d,
				},
				Hex: "014115015c7788b62fc29a27664e312e302e305f303034315dc6",
				Err: nil,
			},
			{
				Pkt: Packet{
					Addr: Addr,
					Cmd:  DataUpCmd,
					Len:  0x25,
					Data: &HistoryData{
						Time:     time.Date(2019, 2, 28, 15, 07, 34, 0, time.Local),
						Interval: 5 * time.Second,
						SDs: []SensorData{
							SensorData{26.4, 66.6, 100.86, 78.0},
							SensorData{26.4, 66.6, 100.86, 78.0},
							SensorData{26.4, 66.6, 100.86, 78.0},
							SensorData{26.4, 66.6, 100.86, 78.0},
							SensorData{26.4, 66.6, 100.86, 78.0},
						},
					},
					Crc: 0x8c48,
				},
				Hex: "014125005c7788b600052fc29a27664e2fc29a27664e2fc29a27664e2fc29a27664e2fc29a27664e488c",
				Err: nil,
			},
			{
				Pkt: Packet{
					Addr: Addr,
					Cmd:  TimeSyncCmd,
					Len:  0x04,
					Data: &TimeSync{
						Time: time.Date(2019, 5, 27, 17, 2, 17, 0, time.Local),
					},
					Crc: 0xab2c,
				},
				Hex: "0145045ceba7992cab",
				Err: nil,
			},
			{
				Pkt: Packet{
					Addr: Addr,
					Cmd:  TimeSyncCmd,
					Len:  0x00,
					Data: &TimeSync{},
					Crc:  0x9012,
				},
				Hex: "0145001290",
				Err: nil,
			},
		}

		for i, test := range tests {
			Convey(fmt.Sprintf("Testing: %d", i), func() {
				b, err := hex.DecodeString(test.Hex)
				if err != nil {
					t.Fatalf("hex.DecodeString(%s) error: %s", test.Hex, err)
				}
				var pkt Packet
				err = pkt.Unmarshal(b)
				So(err, ShouldResemble, test.Err)
				So(pkt, ShouldResemble, test.Pkt)

				if err == nil {
					bd, err := test.Pkt.Marshal()
					So(err, ShouldBeNil)
					So(bd, ShouldResemble, b)
				}
			})
		}
	})
}
