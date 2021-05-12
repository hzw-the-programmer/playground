package util

import (
	"bufio"
	"encoding/binary"
	"fmt"
	"io"
)

type LoraStream struct {
	r *bufio.Reader
	w *bufio.Writer
}

// NewLoraStream returns a LoraStream pointer.
func NewLoraStream(r io.Reader, w io.Writer) *LoraStream {
	return &LoraStream{bufio.NewReader(r), bufio.NewWriter(w)}
}

func (s *LoraStream) ReadMsg() ([]byte, error) {
	lo, ll := 3, 2
	hl := lo + ll

	msg, err := s.r.Peek(hl)
	if err != nil {
		return nil, err
	}

	if msg[0] != 0x0a {
		return nil, fmt.Errorf("LoraStream: begin is 0x%02x", msg[0])
	}
	if msg[1] != 0x01 || msg[2] != 0x02 {
		return nil, fmt.Errorf("LoraStream: ver is 0x%02x%02x", msg[1], msg[2])
	}

	l := int(binary.BigEndian.Uint16(msg[lo:]))

	msg, err = s.r.Peek(hl + l)
	if err != nil {
		return nil, err
	}

	if msg[len(msg)-1] != 0x00 {
		return nil, fmt.Errorf("LoraStream: end is 0x%02x", msg[len(msg)-1])
	}

	s.r.Discard(hl + l)

	return msg[hl : len(msg)-1], nil
}

func (s *LoraStream) WriteMsg(msg []byte) error {
	h := []byte("\x0a\x01\x02\x00\x00")
	binary.BigEndian.PutUint16(h[3:], uint16(len(msg)+1))
	_, err := s.w.Write(h)
	if err != nil {
		return err
	}

	_, err = s.w.Write(msg)
	if err != nil {
		return err
	}

	_, err = s.w.Write([]byte("\x00"))
	if err != nil {
		return err
	}

	err = s.w.Flush()
	if err != nil {
		return err
	}

	return nil
}
