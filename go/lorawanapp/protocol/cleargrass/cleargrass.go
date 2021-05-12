/**
 * Author: Zhiwen He <18676797056@163.com>
 */
package cleargrass

import (
	"encoding/binary"
	"errors"
	"time"

	"github.com/npat-efault/crc16"
)

const Addr = 0x01

const (
	AckCmd        = 0xFF
	DataUpCmd     = 0x41
	ConfigUpCmd   = 0x42
	EventDownCmd  = 0x43
	EventUpCmd    = 0x44
	TimeSyncCmd   = 0x45
	FirmwareCmd   = 0x46
	ConfigDownCmd = 0x47
)

const (
	DataUpHistory  = 0x00
	DataUpRealTime = 0x01
)

var (
	ErrSensorDataNot6Bytes       = errors.New("cleargrass.SensorData.Unmarshal expects 6 bytes")
	ErrRealTimeDataNot21Bytes    = errors.New("cleargrass.RealTimeData.Unmarshal expects 21 bytes")
	ErrNotRealTimeData           = errors.New("cleargrass.RealTimeData.Unmarshal not real time data")
	ErrHistoryDataAtLeast13Bytes = errors.New("cleargrass.HistoryData.Unmarshal expects at least 13 bytes")
	ErrNotHistoryData            = errors.New("cleargrass.HistoryData.Unmarshal not history data")
	ErrPacketAtLeast5Bytes       = errors.New("cleargrass.Packet.Unmarshal expects at least 5 bytes")
)

type Data interface {
	Marshal() ([]byte, error)
	Unmarshal([]byte) error
}

type SensorData struct {
	Temperature float32
	Humility    float32
	Pressure    float32
	Battery     float32
}

func (sd *SensorData) Marshal() ([]byte, error) {
	b := make([]byte, 6)

	t := (uint32(sd.Temperature*10+500) << 20) | (uint32(sd.Humility*10) << 8)
	binary.BigEndian.PutUint32(b, t)
	binary.BigEndian.PutUint16(b[3:], uint16(sd.Pressure*100))
	b[5] = byte(sd.Battery)

	return b, nil
}

func (sd *SensorData) Unmarshal(b []byte) error {
	if len(b) != 6 {
		return ErrSensorDataNot6Bytes
	}

	t := binary.BigEndian.Uint16(b[:2])
	sd.Temperature = float32((t>>4)-500) / 10
	t = binary.BigEndian.Uint16(b[1:3])
	sd.Humility = float32(t&0x0fff) / 10
	t = binary.BigEndian.Uint16(b[3:5])
	sd.Pressure = float32(t) / 100
	sd.Battery = float32(b[5])

	return nil
}

type RealTimeData struct {
	Time time.Time
	SD   SensorData
	Ver  string
}

func (rtd *RealTimeData) Marshal() ([]byte, error) {
	b := make([]byte, 21)

	b[0] = DataUpRealTime
	binary.BigEndian.PutUint32(b[1:], uint32(rtd.Time.Unix()))
	sd, err := rtd.SD.Marshal()
	if err != nil {
		return nil, err
	}
	copy(b[5:], sd)
	copy(b[11:], []byte(rtd.Ver))

	return b, nil
}

func (rtd *RealTimeData) Unmarshal(b []byte) error {
	if len(b) != 21 {
		return ErrRealTimeDataNot21Bytes
	}
	if b[0] != DataUpRealTime {
		return ErrNotRealTimeData
	}

	rtd.Time = time.Unix(int64(binary.BigEndian.Uint32(b[1:])), 0)
	err := rtd.SD.Unmarshal(b[5:11])
	if err != nil {
		return err
	}
	rtd.Ver = string(b[11:])

	return nil
}

type HistoryData struct {
	Time     time.Time
	Interval time.Duration
	SDs      []SensorData
}

func (hd *HistoryData) Marshal() ([]byte, error) {
	b := make([]byte, 1+4+2+len(hd.SDs)*6)

	b[0] = DataUpHistory
	binary.BigEndian.PutUint32(b[1:], uint32(hd.Time.Unix()))
	binary.BigEndian.PutUint16(b[5:], uint16(hd.Interval/time.Second))
	for i, sd := range hd.SDs {
		sdb, err := sd.Marshal()
		if err != nil {
			break
		}
		copy(b[1+4+2+i*6:], sdb)
	}

	return b, nil
}

func (hd *HistoryData) Unmarshal(b []byte) error {
	if len(b) < 13 {
		return ErrHistoryDataAtLeast13Bytes
	}
	if b[0] != DataUpHistory {
		return ErrNotHistoryData
	}

	hd.Time = time.Unix(int64(binary.BigEndian.Uint32(b[1:])), 0)
	hd.Interval = time.Duration(binary.BigEndian.Uint16(b[5:])) * time.Second
	b = b[7:]
	for len(b) >= 6 {
		var sd SensorData
		err := sd.Unmarshal(b[:6])
		if err != nil {
			break
		}
		hd.SDs = append(hd.SDs, sd)
		b = b[6:]
	}

	return nil
}

type TimeSync struct {
	Time time.Time
}

func (ts *TimeSync) Unmarshal(b []byte) error {
	if len(b) == 4 {
		ts.Time = time.Unix(int64(binary.BigEndian.Uint32(b)), 0)
	}
	return nil
}

func (ts *TimeSync) Marshal() ([]byte, error) {
	if ts.Time.IsZero() {
		return []byte{}, nil
	}

	b := make([]byte, 4)

	binary.BigEndian.PutUint32(b, uint32(ts.Time.Unix()))

	return b, nil
}

type Packet struct {
	Addr uint8
	Cmd  uint8
	Len  uint8
	Data Data
	Crc  uint16
}

func (pkt *Packet) Marshal() ([]byte, error) {
	var b []byte

	b = append(b, pkt.Addr)
	switch pkt.Data.(type) {
	case *RealTimeData, *HistoryData:
		b = append(b, DataUpCmd)
	case *TimeSync:
		b = append(b, TimeSyncCmd)
	}

	bd, err := pkt.Data.Marshal()
	if err != nil {
		return nil, err
	}
	b = append(b, byte(len(bd)))
	b = append(b, bd...)

	b = append(b, 0x00, 0x00)
	crci := len(b) - 2
	binary.LittleEndian.PutUint16(b[crci:], crc16.Checksum(crc16.Modbus, b[:crci]))

	return b, nil
}

func (pkt *Packet) Unmarshal(b []byte) error {
	if len(b) < 5 {
		return ErrPacketAtLeast5Bytes
	}

	pkt.Addr = b[0]
	pkt.Cmd = b[1]
	pkt.Len = b[2]
	crci := len(b) - 2
	pkt.Crc = binary.LittleEndian.Uint16(b[crci:])
	b = b[3:crci]
	switch pkt.Cmd {
	case DataUpCmd:
		switch b[0] {
		case DataUpRealTime:
			pkt.Data = &RealTimeData{}
		case DataUpHistory:
			pkt.Data = &HistoryData{}
		}
	case TimeSyncCmd:
		pkt.Data = &TimeSync{}
	}
	if pkt.Data != nil {
		err := pkt.Data.Unmarshal(b)
		if err != nil {
			return err
		}
	}

	return nil
}
