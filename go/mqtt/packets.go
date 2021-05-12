package mqtt

func EncodeLen(len int) []byte {
	var b []byte
	for {
		r := byte(len & 0x7f)
		len >>= 7
		if len > 0 {
			r |= 0x80
		}
		b = append(b, r)
		if len == 0 {
			break
		}
	}
	return b
}
