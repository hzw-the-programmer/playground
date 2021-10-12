package writers

import (
	"bytes"
	"fmt"
	"testing"
)

func TestWidth(t *testing.T) {
	tests := []struct {
		src  string
		cols int
		want string
	}{
		{src: "hello world!", cols: 3, want: "0x68, 0x65, 0x6c,\n0x6c, 0x6f, 0x20,\n0x77, 0x6f, 0x72,\n0x6c, 0x64, 0x21,"},
		{src: "hello world!", cols: 5, want: "0x68, 0x65, 0x6c, 0x6c, 0x6f,\n0x20, 0x77, 0x6f, 0x72, 0x6c,\n0x64, 0x21,"},
	}

	for i, test := range tests {
		name := fmt.Sprintf("%d", i)
		t.Run(name, func(t *testing.T) {
			var buf bytes.Buffer
			h := NewHex(&buf, test.cols)
			w := NewWidth(&buf, h, test.cols)

			w.Write([]byte(test.src))
			dst := buf.String()

			if dst != test.want {
				t.Errorf("\ngot:\n%s\nwant:\n%s", dst, test.want)
			}
		})
	}
}
