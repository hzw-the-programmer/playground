package hzw

import (
	"testing"
	"reflect"
)

func TestEvenlySplitInt64(t *testing.T) {
	tests := []struct {
		name         string
		arr          []int64
		batchMaxSize int
		want         [][]int64
	}{
		{name: "case0", arr: []int64{1}, batchMaxSize: 1, want: [][]int64{{1}}},
		{name: "case1", arr: []int64{1}, batchMaxSize: 2, want: [][]int64{{1}}},
		{name: "case2", arr: []int64{}, batchMaxSize: 2, want: [][]int64{}},
		{name: "case3", arr: []int64{1, 2, 3}, batchMaxSize: 1, want: [][]int64{{1}, {2}, {3}}},
		{name: "case4", arr: []int64{1, 2, 3}, batchMaxSize: 2, want: [][]int64{{1, 2}, {3}}},
		{name: "case5", arr: []int64{1, 2, 3, 4}, batchMaxSize: 2, want: [][]int64{{1, 2}, {3, 4}}},
		{name: "case6", arr: []int64{1, 2, 3, 4, 5}, batchMaxSize: 2, want: [][]int64{{1, 2}, {3, 4}, {5}}},
		{name: "case7", arr: []int64{1, 2, 3, 4, 5, 6}, batchMaxSize: 2, want: [][]int64{{1, 2}, {3, 4}, {5, 6}}},
		{name: "case8", arr: []int64{1, 2, 3, 4, 5, 6}, batchMaxSize: 3, want: [][]int64{{1, 2, 3}, {4, 5, 6}}},
		{name: "case9", arr: []int64{1, 2, 3, 4, 5, 6, 7}, batchMaxSize: 3, want: [][]int64{{1, 2, 3}, {4, 5}, {6, 7}}},
		{name: "case10", arr: []int64{1, 2, 3, 4, 5, 6, 7, 8}, batchMaxSize: 3, want: [][]int64{{1, 2, 3}, {4, 5, 6}, {7, 8}}},
		{name: "case11", arr: []int64{1, 2, 3, 4, 5, 6, 7, 8}, batchMaxSize: 4, want: [][]int64{{1, 2, 3, 4}, {5, 6, 7, 8}}},
		{name: "case12", arr: []int64{1, 2, 3, 4, 5, 6, 7, 8}, batchMaxSize: 5, want: [][]int64{{1, 2, 3, 4}, {5, 6, 7, 8}}},
		{name: "case13", arr: []int64{1, 2, 3, 4, 5, 6, 7, 8}, batchMaxSize: 6, want: [][]int64{{1, 2, 3, 4}, {5, 6, 7, 8}}},
		{name: "case14", arr: []int64{1, 2, 3, 4, 5, 6, 7, 8}, batchMaxSize: 7, want: [][]int64{{1, 2, 3, 4}, {5, 6, 7, 8}}},
		{name: "case15", arr: []int64{1, 2, 3, 4, 5, 6, 7, 8}, batchMaxSize: 8, want: [][]int64{{1, 2, 3, 4, 5, 6, 7, 8}}},
		{name: "case16", arr: []int64{1, 2, 3, 4, 5, 6, 7, 8}, batchMaxSize: 9, want: [][]int64{{1, 2, 3, 4, 5, 6, 7, 8}}},
		{name: "case17", arr: []int64{1, 2, 3, 4, 5, 6, 7, 8, 9}, batchMaxSize: 1, want: [][]int64{{1}, {2}, {3}, {4}, {5}, {6}, {7}, {8}, {9}}},
		{name: "case18", arr: []int64{1, 2, 3, 4, 5, 6, 7, 8, 9}, batchMaxSize: 2, want: [][]int64{{1, 2}, {3, 4}, {5, 6}, {7, 8}, {9}}},
		{name: "case19", arr: []int64{1, 2, 3, 4, 5, 6, 7, 8, 9}, batchMaxSize: 3, want: [][]int64{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}}},
		{name: "case20", arr: []int64{1, 2, 3, 4, 5, 6, 7, 8, 9}, batchMaxSize: 4, want: [][]int64{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}}},
		{name: "case21", arr: []int64{1, 2, 3, 4, 5, 6, 7, 8, 9}, batchMaxSize: 5, want: [][]int64{{1, 2, 3, 4, 5}, {6, 7, 8, 9}}},
		{name: "case22", arr: []int64{1, 2, 3, 4, 5, 6, 7, 8, 9}, batchMaxSize: 6, want: [][]int64{{1, 2, 3, 4, 5}, {6, 7, 8, 9}}},
		{name: "case23", arr: []int64{1, 2, 3, 4, 5, 6, 7, 8, 9}, batchMaxSize: 7, want: [][]int64{{1, 2, 3, 4, 5}, {6, 7, 8, 9}}},
		{name: "case24", arr: []int64{1, 2, 3, 4, 5, 6, 7, 8, 9}, batchMaxSize: 8, want: [][]int64{{1, 2, 3, 4, 5}, {6, 7, 8, 9}}},
		{name: "case25", arr: []int64{1, 2, 3, 4, 5, 6, 7, 8, 9}, batchMaxSize: 9, want: [][]int64{{1, 2, 3, 4, 5, 6, 7, 8, 9}}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := EvenlySplitInt64(tt.arr, tt.batchMaxSize); !reflect.DeepEqual(got, tt.want) {
				t.Errorf("EvenlySplitInt64() = %v, want %v", got, tt.want)
			}
		})
	}
}