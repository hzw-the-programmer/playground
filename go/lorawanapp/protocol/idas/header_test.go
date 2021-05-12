/**
 * Author: Zhiwen He <18676797056@163.com>
 */
package idas

import (
	"encoding/hex"
	"fmt"
	"testing"
	"time"

	. "github.com/smartystreets/goconvey/convey"
)

func TestHeader(t *testing.T) {
	Convey("Given a set of tests", t, func() {
		tests := []struct {
			hex    string
			header Header
			err    error
		}{
			{
				hex: "5434001a00001258390438c101234567120117170b0b",
				header: Header{
					From: 0x54,
					Ver:  0x34,
					Len:  26,
					Sn:   0x00001258390438c1,
					Seq:  0x01234567,
					Time: time.Date(2018, 1, 23, 23, 11, 11, 0, time.Local),
				},
				err: nil,
			},
			{
				hex:    "5434001a00001258390438c10123456712011717",
				header: Header{},
				err:    ErrHeaderLen,
			},
		}

		for i, test := range tests {
			Convey(fmt.Sprintf("Testing: %d", i), func() {
				bs, err := hex.DecodeString(test.hex)
				if err != nil {
					t.Fatal(err)
				}

				var header Header
				err = header.Unmarshal(bs)
				So(err, ShouldResemble, test.err)
				So(header, ShouldResemble, test.header)

				if err == nil {
					hbs, err := test.header.Marshal()
					So(err, ShouldBeNil)
					So(hbs, ShouldResemble, bs)
				}
			})
		}
	})
}
