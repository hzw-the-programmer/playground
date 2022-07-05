package hzw

// go test
// go test -v
// go test -cover
// go test -coverprofile=coverage.out
// go tool cover -html=coverage.out

import (
	"testing"
)

func TestSum(t *testing.T) {
	var tests = []struct {
		a, b, expected int
	} {
		{1, 2, 3},
		{2, 3, 5},
	}

	for i, test := range tests {
		if result := Sum(test.a, test.b); result != test.expected {
			t.Errorf("%d: %d + %d got %d, expected %d", i, test.a, test.b, result, test.expected)
		}
	}
}
