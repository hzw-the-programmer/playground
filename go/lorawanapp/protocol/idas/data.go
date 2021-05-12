/**
 * Author: Zhiwen He <18676797056@163.com>
 */
package idas

type Data interface {
	Marshal() ([]byte, error)
	Unmarshal([]byte) error
}
