package writers

import (
	"bytes"
	"fmt"
	"testing"
)

func TestIdent(t *testing.T) {
	tests := []struct {
		ident  string
		inputs []string
		want   string
	}{
		{
			ident: "    ",
			inputs: []string{
				"#define BEGIN_MACRO begin\n",
				"static const unsigned char en[] = {\n",
				"world!\nI'm Zhiwen He.\n",
				"I'm from China\n",
				"};\n",
				"#define END_MACRO end\n",
			},
			want: `#define BEGIN_MACRO begin
static const unsigned char en[] = {
    world!
    I'm Zhiwen He.
    I'm from China
};
#define END_MACRO end
`,
		},
		{
			ident: "    ",
			inputs: []string{
				`#include "stdio.h"

void process(int i);

`,
				`int main(int argc, char **argv)
{
`,
				`int i;

`,
				`for (i = 0; i < 10; i++)
{
process(i);
}
`,
				`}

`,
				`void process(int i)
{
printf("%d", i);
}
`,
			},
			want: `#include "stdio.h"

void process(int i);

int main(int argc, char **argv)
{
    int i;

    for (i = 0; i < 10; i++)
    {
        process(i);
    }
}

void process(int i)
{
    printf("%d", i);
}
`,
		},
		{
			ident: "    ",
			inputs: []string{
				"#define BEGIN_MACRO begin\r\n",
				"static const unsigned char en[] = {\r\n",
				"world!\r\nI'm Zhiwen He.\r\n",
				"I'm from China\r\n",
				"};\r\n",
				"#define END_MACRO end\r\n",
			},
			want: "#define BEGIN_MACRO begin\r\nstatic const unsigned char en[] = {\r\n    world!\r\n    I'm Zhiwen He.\r\n    I'm from China\r\n};\r\n#define END_MACRO end\r\n",
		},
	}

	for i, test := range tests {
		name := fmt.Sprintf("%d", i)
		t.Run(name, func(t *testing.T) {
			var buf bytes.Buffer

			w := NewIdent(&buf, test.ident)

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
