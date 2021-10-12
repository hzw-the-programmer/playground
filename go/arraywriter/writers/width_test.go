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
		{src: "hello world!", cols: 3, want: "hel\nlo \nwor\nld!"},
		{src: "hello world!", cols: 5, want: "hello\n worl\nd!"},
	}

	for i, test := range tests {
		name := fmt.Sprintf("%d", i)
		t.Run(name, func(t *testing.T) {
			var buf bytes.Buffer
			w := NewWidth(&buf, test.cols)

			w.Write([]byte(test.src))
			got := buf.String()

			if got != test.want {
				t.Errorf("\ngot:\n%s\nwant:\n%s", got, test.want)
			}
		})
	}
}
