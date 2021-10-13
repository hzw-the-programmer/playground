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

func TestHexWidthIdent(t *testing.T) {
	tests := []struct {
		header   string
		contents []string
		footer   string
		want     string
		ident    string
		cols     int
		space    bool
	}{
		{
			header: `static const unsigned char en[] =
{
`, contents: []string{"hello world!"},
			footer: "\n};\n#define SIZE sizeof(en)",
			want: `static const unsigned char en[] =
{
    0x68, 0x65, 0x6c, 0x6c, 0x6f, 
    0x20, 0x77, 0x6f, 0x72, 0x6c, 
    0x64, 0x21, 
};
#define SIZE sizeof(en)`,
			ident: "    ",
			cols:  5,
			space: true,
		},
	}

	for i, test := range tests {
		name := fmt.Sprintf("%d", i)
		t.Run(name, func(t *testing.T) {
			var buf bytes.Buffer

			ident := NewIdent(&buf, test.ident)
			wrapped := NewWidth(ident, test.cols*6)
			wrapped = NewHex(wrapped, true)

			ident.Write([]byte(test.header))

			for _, content := range test.contents {
				wrapped.Write([]byte(content))
			}

			ident.Write([]byte(test.footer))

			got := buf.String()

			if got != test.want {
				t.Errorf("\ngot:\n%s\nwant:\n%s", got, test.want)
			}
		})
	}
}
