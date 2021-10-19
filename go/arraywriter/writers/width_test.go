package writers

import (
	"bytes"
	"fmt"
	"testing"
)

func init() {
	NL = []byte{'\n'}
}

func TestWidth(t *testing.T) {
	tests := []struct {
		inputs []string
		cols   int
		want   string
	}{
		{inputs: []string{"hello world!"}, cols: 3, want: "hel\nlo \nwor\nld!"},
		{inputs: []string{"hello world!"}, cols: 5, want: "hello\n worl\nd!"},
	}

	for i, test := range tests {
		name := fmt.Sprintf("%d", i)
		t.Run(name, func(t *testing.T) {
			var buf bytes.Buffer

			wrapped := NewWidth(&buf, test.cols)

			for _, input := range test.inputs {
				wrapped.Write([]byte(input))
			}

			got := buf.String()

			if got != test.want {
				t.Errorf("\ngot:\n%s\nwant:\n%s", got, test.want)
			}
		})
	}
}

func TestHexWidth(t *testing.T) {
	tests := []struct {
		inputs []string
		cols   int
		want   string
		space  bool
	}{
		{inputs: []string{"hello world!"}, cols: 3 * 5, want: "0x68,0x65,0x6c,\n0x6c,0x6f,0x20,\n0x77,0x6f,0x72,\n0x6c,0x64,0x21,", space: false},
		{inputs: []string{"hello world!"}, cols: 5 * 5, want: "0x68,0x65,0x6c,0x6c,0x6f,\n0x20,0x77,0x6f,0x72,0x6c,\n0x64,0x21,", space: false},
		{inputs: []string{"hello world!"}, cols: 3 * 6, want: "0x68, 0x65, 0x6c, \n0x6c, 0x6f, 0x20, \n0x77, 0x6f, 0x72, \n0x6c, 0x64, 0x21, ", space: true},
		{inputs: []string{"hello world!"}, cols: 5 * 6, want: "0x68, 0x65, 0x6c, 0x6c, 0x6f, \n0x20, 0x77, 0x6f, 0x72, 0x6c, \n0x64, 0x21, ", space: true},
	}

	for i, test := range tests {
		name := fmt.Sprintf("%d", i)
		t.Run(name, func(t *testing.T) {
			var buf bytes.Buffer

			wrapped := NewWidth(&buf, test.cols)
			wrapped = NewHex(wrapped, test.space)

			for _, input := range test.inputs {
				wrapped.Write([]byte(input))
			}

			got := buf.String()

			if got != test.want {
				t.Errorf("\ngot:\n%s\nwant:\n%s", got, test.want)
			}
		})
	}
}
