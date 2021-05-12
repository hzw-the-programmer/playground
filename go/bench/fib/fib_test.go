package fib

import (
	"testing"
)

func benchmarkFib(n int, b *testing.B) {
	for i := 0; i < b.N; i++ {
		Fib(n)
	}
}

func BenchmarkFib1(b *testing.B) {
	benchmarkFib(1, b)
}

func BenchmarkFib2(b *testing.B) {
	benchmarkFib(2, b)
}

func BenchmarkFib3(b *testing.B) {
	benchmarkFib(3, b)
}

func BenchmarkFib10(b *testing.B) {
	benchmarkFib(10, b)
}

func BenchmarkFib20(b *testing.B) {
	benchmarkFib(20, b)
}

func BenchmarkFib40(b *testing.B) {
	benchmarkFib(40, b)
}

// func BenchmarkFibWrong(b *testing.B) {
// 	for i := 0; i < b.N; i++ {
// 		Fib(i)
// 	}
// }

// func BenchmarkFibWrong2(b *testing.B) {
// 	Fib(b.N)
// }

var result int

func BenchmarkFibComplete(b *testing.B) {
	var r int
	for i := 0; i < b.N; i++ {
		r = Fib(10)
	}
	result = r
}

var fibTests = []struct {
	n        int
	expected int
}{
	{1, 1},
	{2, 1},
	{3, 2},
	{4, 3},
	{5, 5},
	{6, 8},
	{7, 13},
}

func TestFib(t *testing.T) {
	for _, ft := range fibTests {
		actual := Fib(ft.n)
		if actual != ft.expected {
			t.Errorf("Fib(%d): expected %d, actual %d", ft.n, ft.expected, actual)
		}
	}
}
