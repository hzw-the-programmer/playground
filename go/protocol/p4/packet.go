/**
 * Author: Zhiwen He <18676797056@163.com>
 */
package p4

import (
	"encoding/binary"
	"errors"
)

var (
	ErrPacketBodyType = errors.New("idas.Packet.Unmarshal: wrong body type")
	ErrPacketChksum   = errors.New("idas.Packet.Unmarshal: wrong checksum")
)

type Packet struct {
	Header *Header
	Body   Body
}

func (packet *Packet) Marshal() ([]byte, error) {
	var bs []byte

	hbs, err := packet.Header.Marshal()
	if err != nil {
		return nil, err
	}

	bbs, err := packet.Body.Marshal()
	if err != nil {
		return nil, err
	}

	bs = append(bs, hbs...)
	bs = append(bs, bbs...)
	bs = append(bs, Chksum(bs[4:]))
	binary.BigEndian.PutUint16(bs[2:4], uint16(len(bs[4:])))

	return bs, nil
}

func (packet *Packet) Unmarshal(bs []byte) error {
	var header Header
	var body Body

	err := header.Unmarshal(bs)
	if err != nil {
		return err
	}
	packet.Header = &header

	switch bs[HeaderLen] {
	case BodyTypeHeartBeatWithData:
		body = &HeartBeatWithData{}
	default:
		return ErrPacketBodyType
	}

	err = body.Unmarshal(bs[HeaderLen : len(bs)-1])
	if err != nil {
		return err
	}
	packet.Body = body

	if Chksum(bs[4:len(bs)-1]) != bs[len(bs)-1] {
		return ErrPacketChksum
	}

	return nil
}
