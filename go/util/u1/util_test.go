package util

import (
	"testing"
)

func TestIsSame(t *testing.T) {
	tests := []struct {
		name  string
		phone string
		same  bool
	}{{
		name: "861******7592",
		phone: "861234166759",
	}, {
		name: "861******75921",
		phone: "8612341667592",
	}, {
		name:  "861**1***7592",
		phone: "8612341667592",
	}, {
		name:  "861******7592",
		phone: "8612341667592",
		same:  true,
	}}

	for _, test := range tests {
		if IsSame(test.name, test.phone) != test.same {
			t.Fatal("not equal")
		}
	}
}
