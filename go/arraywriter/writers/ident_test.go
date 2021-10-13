package writers

import (
	"bytes"
	"fmt"
	"testing"
)

func TestIdent(t *testing.T) {
	tests := []struct {
		inputs []string
		want   string
		char   byte
		repeat int
		nl     bool
	}{
		{
			inputs: []string{
				"hello ",
				"world!\nI'm Zhiwen He.\n",
				"I'm from China\n",
			},
			want:   "hello world!\n    I'm Zhiwen He.\n    I'm from China\n",
			char:   ' ',
			repeat: 4,
			nl:     false,
		},
	}

	for i, test := range tests {
		name := fmt.Sprintf("%d", i)
		t.Run(name, func(t *testing.T) {
			var buf bytes.Buffer
			w := NewIdent(&buf, test.char, test.repeat, test.nl)
			for _, input := range test.inputs {
				w.Write([]byte(input))
			}
			got := buf.String()
			if got != test.want {
				t.Errorf("\ngot:\n%s\nwant:\n%s", got, test.want)
			}
		})
	}
}
