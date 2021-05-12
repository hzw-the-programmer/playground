/**
 * Author: Zhiwen He <18676797056@163.com>
 */
package p4

const (
	ChanTypeGndH       = 0x08
	ChanTypeWS         = 0x09
	ChanTypeVB         = 0x0a
	ChanTypeTemp       = 0x0b
	ChanTypeGndL       = 0x0c
	ChanTypeHumi       = 0x0d
	ChanTypeGndL1      = 0x0e
	ChanTypeGndL10     = 0x0f
	ChanTypeEsiV       = 0x10
	ChanTypeEsiR       = 0x11
	ChanTypeEsdV       = 0x12
	ChanTypeEsdR       = 0x13
	ChanTypeSmoke      = 0x14
	ChanTypeWeight     = 0x15
	ChanTypeDoorMagnet = 0x16
	ChanTypeSocket     = 0x17
	ChanTypeLight      = 0x18
	ChanTypeMagnet     = 0x19
	ChanTypeWell       = 0x20
	ChanTypeDoorHolder = 0x21
)

type Channel struct {
	Slot    uint8
	Port    uint8
	Type    uint8
	Status  int
	Data    float32
	AvgData float32
}
