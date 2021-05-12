/**
 * Author: Zhiwen He <18676797056@163.com>
 */
package p4

import (
	"encoding/binary"
	"errors"
	"time"
)

const (
	HeaderLen = 22
)

var (
	ErrHeaderLen = errors.New("idas.Header.Unmarshal: wrong len")
)

type Header struct {
	From byte
	Ver  byte
	Len  uint16
	Sn   uint64
	Seq  uint32
	Time time.Time
}

func (header *Header) Marshal() ([]byte, error) {
	bs := make([]byte, HeaderLen)

	bs[0] = header.From
	bs[1] = header.Ver
	binary.BigEndian.PutUint16(bs[2:4], header.Len)
	binary.BigEndian.PutUint64(bs[4:12], header.Sn)
	binary.BigEndian.PutUint32(bs[12:16], header.Seq)
	bs[16] = byte(header.Time.Year() - 2000)
	bs[17] = byte(header.Time.Month())
	bs[18] = byte(header.Time.Day())
	bs[19] = byte(header.Time.Hour())
	bs[20] = byte(header.Time.Minute())
	bs[21] = byte(header.Time.Second())

	return bs, nil
}

func (header *Header) Unmarshal(bs []byte) error {
	if len(bs) < HeaderLen {
		return ErrHeaderLen
	}

	header.From = bs[0]
	header.Ver = bs[1]
	header.Len = binary.BigEndian.Uint16(bs[2:4])
	header.Sn = binary.BigEndian.Uint64(bs[4:12])
	header.Seq = binary.BigEndian.Uint32(bs[12:16])
	header.Time = time.Date(
		int(bs[16])+2000, time.Month(bs[17]), int(bs[18]),
		int(bs[19]), int(bs[20]), int(bs[21]), 0, time.Local)

	return nil
}
