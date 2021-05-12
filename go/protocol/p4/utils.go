/**
 * Author: Zhiwen He <18676797056@163.com>
 */
package p4

func Chksum(bs []byte) byte {
	var chksum byte
	for _, b := range bs {
		chksum ^= b
	}
	return chksum
}
