package lora

import (
	"bufio"
	"encoding/binary"
	"io"
	"net"
)

type Conn struct {
	net.Conn
	writer *bufio.Writer
}

func NewConn(conn net.Conn) *Conn {
	return &Conn{
		Conn:   conn,
		writer: bufio.NewWriter(conn),
	}
}

func (mc *Conn) ReadBd() ([]byte, error) {
	hd := [5]byte{}
	_, err := io.ReadFull(mc, hd[:])
	if err != nil {
		return nil, err
	}

	bl := binary.BigEndian.Uint16(hd[3:])
	bd := make([]byte, bl)
	_, err = io.ReadFull(mc, bd)
	if err != nil {
		return nil, err
	}

	return bd[:bl-1], nil
}

func (mc *Conn) WriteBd(bd []byte) error {
	var hd = []byte("\x0a\x01\x02\x00\x00")
	var tl = []byte("\x00")

	binary.BigEndian.PutUint16(hd[3:], uint16(len(bd)+1))
	_, err := mc.writer.Write(hd)
	if err != nil {
		return err
	}

	_, err = mc.writer.Write(bd)
	if err != nil {
		return err
	}

	_, err = mc.writer.Write(tl)
	if err != nil {
		return err
	}

	err = mc.writer.Flush()
	if err != nil {
		return err
	}

	return nil
}
