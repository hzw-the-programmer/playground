package lorawan

import (
	"testing"

	. "github.com/smartystreets/goconvey/convey"
)

func TestChMask(t *testing.T) {
	Convey("Given an empty ChMask", t, func() {
		var cm ChMask

		Convey("Then MarshalBinary returns []byte{0, 0}", func() {
			b, err := cm.MarshalBinary()
			So(err, ShouldBeNil)
			So(b, ShouldResemble, []byte{0, 0})
		})

		Convey("Given channel 1, 2 and 13 are set to true", func() {
			cm[0] = true
			cm[1] = true
			cm[12] = true
			Convey("Then MarshalBinary returns []byte{0x03, 0x10}", func() {
				b, err := cm.MarshalBinary()
				So(err, ShouldBeNil)
				So(b, ShouldResemble, []byte{0x03, 0x10})
			})
		})

		Convey("Given a slice of 3 bytes", func() {
			b := []byte{0x01, 0x02, 0x03}
			Convey("Then UnmarshalBinary returns an error", func() {
				err := cm.UnmarshalBinary(b)
				So(err, ShouldNotBeNil)
			})
		})

		Convey("Given the slice []byte{0x03, 0x10}", func() {
			b := []byte{0x03, 0x10}
			Convey("Then UnmarshalBinary returns a ChMask with channel 1, 2, and 13 set to true", func() {
				err := cm.UnmarshalBinary(b)
				var exp ChMask
				exp[0] = true
				exp[1] = true
				exp[12] = true
				So(err, ShouldBeNil)
				So(cm, ShouldResemble, exp)
			})
		})
	})
}
