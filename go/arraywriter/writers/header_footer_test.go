package writers

import (
	"bytes"
	"fmt"
	"io"
	"testing"
)

func TestHeaderFooter(t *testing.T) {
	tests := []struct {
		header  string
		content string
		footer  string
		cols    int
		hex     bool
		want    string
	}{
		{
			header:  "static const unsigned char en = {\n",
			content: "hello world!",
			footer:  "\n};",
			cols:    7,
			hex:     true,
			want: `static const unsigned char en = {
0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77,
0x6f, 0x72, 0x6c, 0x64, 0x21,
};`,
		},
		{
			header:  "static const unsigned char en = {\n",
			content: "hello world!",
			footer:  "\n};",
			cols:    7,
			hex:     false,
			want: `static const unsigned char en = {
hello w
orld!
};`,
		},
	}

	for i, test := range tests {
		name := fmt.Sprintf("%d", i)
		t.Run(name, func(t *testing.T) {
			var buf bytes.Buffer
			var hex io.Writer
			var width io.Writer

			hex = &buf
			if test.hex {
				hex = NewHex(&buf, test.cols)
			}

			width = NewWidth(&buf, hex, test.cols)
			w := NewHeaderFooter(&buf, width, test.header, test.footer)

			w.Write([]byte(test.content))
			w.Close()

			got := buf.String()

			if got != test.want {
				t.Errorf("\ngot:\n%s\nwant:\n%s", got, test.want)
			}
		})
	}
}
