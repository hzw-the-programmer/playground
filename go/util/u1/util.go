package util

func IsSame(name, phone string) bool {
	n := []rune(name)
	p := []rune(phone)

	if len(n) != len(p) {
		return false
	}

	for i, r := range n {
		if r != '*' && r != p[i] {
			return false
		}
	}

	return true
}
