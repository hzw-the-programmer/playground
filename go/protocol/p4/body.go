/**
 * Author: Zhiwen He <18676797056@163.com>
 */
package p4

import (
	"encoding/binary"
	"errors"
	"math"
)

const (
	BodyTypeSetSn             = 0x05
	BodyTypeGetSn             = 0x06
	BodyTypeSetWorkTime       = 0x07
	BodyTypeGetWorkTime       = 0x08
	BodyTypeCalibration       = 0x09
	BodyTypeDeviceStandbyHB   = 0x0a
	BodyTypeBattery           = 0x0b
	BodyTypeAck               = 0x24
	BodyTypeStatusData        = 0x40
	BodyTypeData              = 0x41
	BodyTypePowerOff          = 0x42
	BodyTypePowerOn           = 0x43
	BodyTypeDeviceInfo        = 0x44
	BodyTypeHeartBeatWithData = 0x45
	BodyTypeOpen              = 0x49
	BodyTypeClose             = 0x50
	BodyTypeStatus            = 0x51
	BodyTypeGetParam          = 0x52
	BodyTypeSetParam          = 0x53
	BodyTypeSmartBox          = 0x60
	BodyTypeP5Cmd             = 0x61
	BodyTypeHeartBeat         = 0xFE
	BodyTypeNone              = 0xFF
)

var (
	ErrNotHeartBeatWithData           = errors.New("idas.HeartBeatWithData.Unmarshal: not HeartBeatWithData")
	ErrHeartBeatWithDataAtLeast6Bytes = errors.New("idas.HeartBeatWithData.Unmarshal: at least 6 bytes")
	ErrHeartBeatWithDataLen           = errors.New("idas.HeartBeatWithData.Unmarshal: wrong len")

	ErrNotStatusData           = errors.New("idas.StatusData.Unmarshal: not StatusData")
	ErrStatusDataAtLeast2Bytes = errors.New("idas.StatusData.Unmarshal: at least 2 bytes")
	ErrStatusDataLen           = errors.New("idas.StatusData.Unmarshal: wrong len")
)

type Body interface {
	Data
	Type() int
	Data() Data
}

type HeartBeatWithData struct {
	Seq      uint32
	Channels []Channel
}

func (this *HeartBeatWithData) Marshal() ([]byte, error) {
	l := len(this.Channels)
	bs := make([]byte, 1+4+1+l*12)

	bs[0] = byte(BodyTypeHeartBeatWithData)
	binary.LittleEndian.PutUint32(bs[1:5], this.Seq)
	bs[5] = byte(l)
	tbs := bs[6:]
	for i, channel := range this.Channels {
		i = i * 12
		tbs[i] = byte(channel.Slot)
		tbs[i+1] = byte(channel.Port)
		tbs[i+2] = byte(channel.Type)
		tbs[i+3] = byte(channel.Status)
		binary.LittleEndian.PutUint32(tbs[i+4:], math.Float32bits(channel.Data))
		binary.LittleEndian.PutUint32(tbs[i+8:], math.Float32bits(channel.AvgData))
	}

	return bs, nil
}

func (this *HeartBeatWithData) Unmarshal(bs []byte) error {
	if bs[0] != BodyTypeHeartBeatWithData {
		return ErrNotHeartBeatWithData
	}

	if len(bs) < 6 {
		return ErrHeartBeatWithDataAtLeast6Bytes
	}

	l := int(bs[5])
	if len(bs) != 1+4+1+l*12 {
		return ErrHeartBeatWithDataLen
	}

	this.Seq = binary.LittleEndian.Uint32(bs[1:5])
	tbs := bs[6:]
	for i := 0; i < l; i++ {
		j := i * 12
		var channel Channel
		channel.Slot = uint8(tbs[j])
		channel.Port = uint8(tbs[j+1])
		channel.Type = uint8(tbs[j+2])
		channel.Status = int(tbs[j+3])
		channel.Data = math.Float32frombits(binary.LittleEndian.Uint32(tbs[j+4:]))
		channel.AvgData = math.Float32frombits(binary.LittleEndian.Uint32(tbs[j+8:]))
		this.Channels = append(this.Channels, channel)
	}

	return nil
}

func (this *HeartBeatWithData) Type() int {
	return BodyTypeHeartBeatWithData
}

func (this *HeartBeatWithData) Data() Data {
	return nil
}

type StatusData struct {
	Channels []Channel
}

func (this *StatusData) Marshal() ([]byte, error) {
	l := len(this.Channels)
	bs := make([]byte, 1+1+l*7)

	bs[0] = byte(BodyTypeStatusData)
	bs[1] = byte(l)
	tbs := bs[2:]
	for i, channel := range this.Channels {
		i *= 7
		tbs[i] = byte(channel.Slot)<<4 | byte(channel.Port)
		tbs[i+1] = byte(channel.Type)
		tbs[i+2] = byte(channel.Status)
		binary.LittleEndian.PutUint32(tbs[i+3:], math.Float32bits(channel.Data))
	}

	return bs, nil
}

func (this *StatusData) Unmarshal(bs []byte) error {
	if bs[0] != BodyTypeStatusData {
		return ErrNotStatusData
	}

	if len(bs) < 2 {
		return ErrStatusDataAtLeast2Bytes
	}

	l := int(bs[1])
	if len(bs) != 1+1+l*7 {
		return ErrStatusDataLen
	}

	tbs := bs[2:]
	for i := 0; i < l; i++ {
		j := i * 7
		var channel Channel
		channel.Slot = tbs[j] >> 4
		channel.Port = tbs[j] & 0x0f
		channel.Type = tbs[j+1]
		channel.Status = int(tbs[j+2])
		channel.Data = math.Float32frombits(binary.LittleEndian.Uint32(tbs[j+3:]))
		this.Channels = append(this.Channels, channel)
	}

	return nil
}

func (this *StatusData) Type() int {
	return BodyTypeStatusData
}

func (this *StatusData) Data() Data {
	return nil
}
