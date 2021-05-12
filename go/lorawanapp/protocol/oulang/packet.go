/**
 * Author: Zhiwen He <18676797056@163.com>
 */
package oulang

import (
	"encoding/binary"
	"errors"
	"math"
)

const (
	PTHeartBeat = 0x01
	PTStatus    = 0x42
	CTGNDH      = 0x08
	CTWS        = 0x09
)

var (
	ErrAtLeast4Bytes   = errors.New("oulang.Packet.Unmarshal: at least 4 bytes")
	ErrWrongTerminator = errors.New("oulang.Packet.Unmarshal: wrong terminator")
)

type Packet struct {
	Battery     int
	Temperature int
	Type        int
	Channel     uint8
	CType       uint8
	Status      int
	Data        float32
}

func (pkt *Packet) Marshal() ([]byte, error) {
	var bs []byte
	switch pkt.Type {
	case PTStatus:
		bs = make([]byte, 11)
		bs[0] = byte(pkt.Battery)
		bs[1] = byte(pkt.Temperature)
		bs[2] = byte(pkt.Type)
		bs[3] = byte(pkt.Channel)
		bs[4] = byte(pkt.CType)
		bs[5] = byte(pkt.Status)
		binary.LittleEndian.PutUint32(bs[6:], math.Float32bits(pkt.Data))
		bs[10] = 0xaa
	case PTHeartBeat:
		bs = make([]byte, 4)
		bs[0] = byte(pkt.Battery)
		bs[1] = byte(pkt.Temperature)
		bs[2] = byte(pkt.Type)
		bs[3] = 0xaa
	}
	return bs, nil
}

func (pkt *Packet) Unmarshal(bs []byte) error {
	l := len(bs)

	if l < 4 {
		return ErrAtLeast4Bytes
	}
	if bs[l-1] != 0xaa {
		return ErrWrongTerminator
	}

	pkt.Battery = int(bs[0])
	pkt.Temperature = int(bs[1])
	pkt.Type = int(bs[2])
	switch pkt.Type {
	case PTStatus:
		pkt.Channel = uint8(bs[3])
		pkt.CType = uint8(bs[4])
		pkt.Status = int(bs[5])
		pkt.Data = math.Float32frombits(binary.LittleEndian.Uint32(bs[6:]))
	}

	return nil
}
