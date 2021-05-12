package ptest

import (
	"encoding/binary"
	"errors"
)

const (
	ACK      = 0x02
	ACK_SIZE = 6
)

type Ack struct {
	Cmd byte
	Seq uint32
}

var (
	ErrAckWrongSize = errors.New("Ack.UnmarshalBinary: wrong size")
	ErrNotAck       = errors.New("Ack.UnmarshalBinary: not ack")
)

func (a Ack) MarshalBinary() ([]byte, error) {
	data := make([]byte, ACK_SIZE)

	data[0] = ACK
	data[1] = a.Cmd
	binary.BigEndian.PutUint32(data[2:], a.Seq)

	return data, nil
}

func (a *Ack) UnmarshalBinary(data []byte) error {
	if len(data) != ACK_SIZE {
		return ErrAckWrongSize
	}

	if data[0] != ACK {
		return ErrNotAck
	}

	a.Cmd = data[1]
	a.Seq = binary.BigEndian.Uint32(data[2:])

	return nil
}
