package writers

// import (
// 	"bytes"
// 	"fmt"
// 	"io"
// 	"testing"
// )

// func TestHeaderFooter(t *testing.T) {
// 	tests := []struct {
// 		header  string
// 		contents []string
// 		footer  string
// 		ident bool
// 		char byte
// 		repeat int
// 		nl bool
// 		hex     bool
// 		width bool
// 		cols    int
// 		want    string
// 	}{
// 		{
// 			header:  "static const unsigned char en = {\n",
// 			contents: []string{"hello world!"},
// 			footer:  "\n};",
// 			ident: true,
// 			char: ' ',
// 			repeat: 4,
// 			nl: true,
// 			hex:     true,
// 			width: true,
// 			cols:    7,
// 			want: `static const unsigned char en = {
//     0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77,
//     0x6f, 0x72, 0x6c, 0x64, 0x21,
// };`,
// 		},
// 	}

// 	for i, test := range tests {
// 		name := fmt.Sprintf("%d", i)
// 		t.Run(name, func(t *testing.T) {
// 			var buf bytes.Buffer
// 			var raw io.Writer
// 			var wrapped io.Writer
// 			var ident io.Writer
// 			var width io.Writer
// 			var hex io.Writer

// 			raw = &buf

// 			if test.ident {
// 				raw = NewIdent(raw, test.char, test.repeat, test.nl)
// 			}

// 			if test.width {
// 				wrapped = NewWidth(ident, hex, test.cols)
// 			}

// 			if test.hex {
// 				wrapped = NewHex(wrapped, test.cols)
// 			}

// 			w := NewHeaderFooter(raw, wrapped, test.header, test.footer)

// 			for _, content := range test.contents {
// 				w.Write([]byte(content))
// 			}

// 			w.Close()

// 			got := buf.String()

// 			if got != test.want {
// 				t.Errorf("\ngot:\n%s\nwant:\n%s", got, test.want)
// 			}
// 		})
// 	}
// }
