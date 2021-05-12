/**
 * Author: Zhiwen He <18676797056@163.com>
 */
package p4

type Data interface {
	Marshal() ([]byte, error)
	Unmarshal([]byte) error
}
