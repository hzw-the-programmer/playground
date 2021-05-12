package fib

func Fib(n int) int {
	if n < 2 {
		return n
	}

	return Fib(n-2) + Fib(n-1)
}
