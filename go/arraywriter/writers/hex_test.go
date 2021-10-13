package writers

import (
	"bytes"
	"fmt"
	"testing"
)

func TestHex(t *testing.T) {
	tests := []struct {
		inputs []string
		want   string
		space  bool
	}{
		{inputs: []string{"hello world!"}, want: "0x68,0x65,0x6c,0x6c,0x6f,0x20,0x77,0x6f,0x72,0x6c,0x64,0x21,", space: false},
		{inputs: []string{"hello world!"}, want: "0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21, ", space: true},
	}

	for i, test := range tests {
		name := fmt.Sprintf("%d", i)
		t.Run(name, func(t *testing.T) {
			var buf bytes.Buffer

			wrapped := NewHex(&buf, test.space)

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
