package algorithm

func MinSubArrayLen(t int, a []int) int {
	var l, r, sum, result int

	for r = 0; r < len(a); r++ {
		sum += a[r]

		for sum >= t {
			if result == 0 || result > r-l+1 {
				result = r - l + 1
			}
			sum -= a[l]
			l++
		}
	}

	return result
}
