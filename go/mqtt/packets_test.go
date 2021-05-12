package mqtt

import (
	"fmt"
	"testing"

	. "github.com/smartystreets/goconvey/convey"
)

func TestEncodeLen(t *testing.T) {
	Convey("Given a set of tests", t, func() {
		tests := []struct {
			len   int
			bytes []byte
		}{
			{0x7f, []byte{0x7f}},
			{0x80, []byte{0x80, 0x01}},
			{0x0100, []byte{0x80, 0x02}},
		}
		for i, test := range tests {
			Convey(fmt.Sprintf("Testing: %d", i), func() {
				So(EncodeLen(test.len), ShouldResemble, test.bytes)
			})
		}
	})
}

func TestDecodeLen(t *testing.T) {

}
