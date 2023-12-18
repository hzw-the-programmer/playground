package algorithm

import "testing"

func TestMinSubArrayLen(t *testing.T) {
	var tests = []struct {
		t    int
		a    []int
		want int
	}{
		{7, []int{2, 3, 1, 2, 4, 3}, 2},
		{4, []int{1, 4, 4}, 1},
		{11, []int{1, 1, 1, 1, 1, 1, 1, 1}, 0},
	}

	for i, test := range tests {
		got := MinSubArrayLen(test.t, test.a)
		if got != test.want {
			t.Fatalf("test %d: want = %d, got = %d", i, test.want, got)
		}
	}
}
