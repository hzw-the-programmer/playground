package util

import (
	"bufio"
	"bytes"
	"fmt"
	"io"
	"strings"
	"testing"
)

func TestReadMsg(t *testing.T) {
	tests := []struct {
		in  string
		out []string
		err error
	}{
		{
			in:  "",
			out: nil,
			err: io.EOF,
		},
		{
			in:  "a",
			out: nil,
			err: io.EOF,
		},
		{
			in:  "abcde",
			out: nil,
			err: fmt.Errorf("LoraStream: begin is 0x%02x", 'a'),
		},
		{
			in:  "\x0abcde",
			out: nil,
			err: fmt.Errorf("LoraStream: ver is 0x%02x%02x", 'b', 'c'),
		},
		{
			in:  "\x0a\x01\x02\x0f\xfe",
			out: nil,
			err: bufio.ErrBufferFull,
		},
		{
			in:  "\x0a\x01\x02\x0f\xfb",
			out: nil,
			err: io.EOF,
		},
		{
			in:  "\x0a\x01\x02\x00\x02a",
			out: nil,
			err: io.EOF,
		},
		{
			in:  "\x0a\x01\x02\x00\x02ab",
			out: nil,
			err: fmt.Errorf("LoraStream: end is 0x%02x", 'b'),
		},
		{
			in:  "\x0a\x01\x02\x00\x02a\x00",
			out: []string{"a"},
			err: io.EOF,
		},
		{
			in:  "\x0a\x01\x02\x00\x02a\x00\x0a\x01\x02\x00\x03ab\x00",
			out: []string{"a", "ab"},
			err: io.EOF,
		},
		{
			in: "\x0a\x01\x02\x00\x02a\x00\x0a\x01\x02\x00\x03ab" +
				"\x00\x0a\x01\x02\x00\x0chello world\x00",
			out: []string{"a", "ab", "hello world"},
			err: io.EOF,
		},
		{
			in: "\x0a\x01\x02\x00\x02a\x00\x0a\x01\x02\x00\x03ab\x00" +
				"\x0a\x01\x02\x00\x0chello world\x00" +
				"\x0b\x01\x02\x00",
			out: []string{"a", "ab", "hello world"},
			err: io.EOF,
		},
		{
			in: "\x0a\x01\x02\x00\x02a\x00\x0a\x01\x02\x00\x03ab\x00" +
				"\x0a\x01\x02\x00\x0chello world\x00" +
				"\x0b\x01\x02\x00\x0c",
			out: []string{"a", "ab", "hello world"},
			err: fmt.Errorf("LoraStream: begin is 0x%02x", '\x0b'),
		},
		{
			in: "\x0a\x01\x02\x00\x02a\x00\x0a\x01\x02\x00\x03ab\x00" +
				"\x0a\x01\x02\x00\x0chello world\x00" +
				"\x0a\x02\x02\x00\x0c",
			out: []string{"a", "ab", "hello world"},
			err: fmt.Errorf("LoraStream: ver is 0x%02x%02x", '\x02', '\x02'),
		},
		{
			in: "\x0a\x01\x02\x00\x02a\x00\x0a\x01\x02\x00\x03ab\x00" +
				"\x0a\x01\x02\x00\x0chello world\x00" +
				"\x0a\x01\x02\x0f\xfc",
			out: []string{"a", "ab", "hello world"},
			err: bufio.ErrBufferFull,
		},
		{
			in: "\x0a\x01\x02\x00\x02a\x00\x0a\x01\x02\x00\x03ab\x00" +
				"\x0a\x01\x02\x00\x0chello world\x00" +
				"\x0a\x01\x02\x0f\xfb",
			out: []string{"a", "ab", "hello world"},
			err: io.EOF,
		},
		{
			in: "\x0a\x01\x02\x00\x02a\x00\x0a\x01\x02\x00\x03ab\x00" +
				"\x0a\x01\x02\x00\x0chello world\x00" +
				"\x0a\x01\x02\x00\x05abcde",
			out: []string{"a", "ab", "hello world"},
			err: fmt.Errorf("LoraStream: end is 0x%02x", 'e'),
		},
		{
			in: "\x0a\x01\x02\x00\x02a\x00\x0a\x01\x02\x00\x03ab\x00" +
				"\x0a\x01\x02\x00\x0chello world\x00" +
				"\x0a\x01\x02\x00\x05abcd\x00",
			out: []string{"a", "ab", "hello world", "abcd"},
			err: io.EOF,
		},
	}

	for _, test := range tests {
		s := NewLoraStream(strings.NewReader(test.in), nil)
		i := 0
		for {
			msg, err := s.ReadMsg()
			if err != nil {
				if err.Error() != test.err.Error() {
					t.Errorf("expected error %v, got %v", test.err, err)
				}
				break
			}
			if string(msg) != test.out[i] {
				t.Errorf("expected %q, got %q", test.out[i], msg)
			}
			i++
		}
	}
}

func TestWriteMsg(t *testing.T) {
	tests := []struct {
		in  []string
		out string
	}{
		{
			in:  []string{"hello world"},
			out: "\x0a\x01\x02\x00\x0chello world\x00",
		},
		{
			in: []string{"hello world", "hzw"},
			out: "\x0a\x01\x02\x00\x0chello world\x00" +
				"\x0a\x01\x02\x00\x04hzw\x00",
		},
	}

	for _, test := range tests {
		b := new(bytes.Buffer)
		s := NewLoraStream(nil, b)
		for _, in := range test.in {
			s.WriteMsg([]byte(in))
		}
		if string(b.Bytes()) != test.out {
			t.Errorf("expected %q, got %q", test.out, b.Bytes())
		}
	}
}
