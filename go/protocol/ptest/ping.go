package ptest

import (
	"encoding/binary"
	"errors"
)

const (
	PING      = 0x01
	PING_SIZE = 5
)

var (
	ErrPingWrongSize = errors.New("Ping.UnmarshalBinary: wrong size")
	ErrNotPing       = errors.New("Ping.UnmarshalBinary: not ping")
)

type Ping struct {
	Seq uint32
}

func (p Ping) MarshalBinary() ([]byte, error) {
	data := make([]byte, PING_SIZE)

	data[0] = PING
	binary.BigEndian.PutUint32(data[1:], p.Seq)

	return data, nil
}

func (p *Ping) UnmarshalBinary(data []byte) error {
	if len(data) != PING_SIZE {
		return ErrPingWrongSize
	}

	if data[0] != PING {
		return ErrNotPing
	}

	p.Seq = binary.BigEndian.Uint32(data[1:])

	return nil
}
