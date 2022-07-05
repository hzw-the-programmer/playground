package hzw

func EvenlySplitInt64(array []int64, batchSize int) [][]int64 {
	if len(array) == 0 {
		return [][]int64{}
	}

	// count := (len(array) + batchSize - 1) / batchSize
	count := (len(array) - 1) / batchSize + 1
	
	splits := make([][]int64, count)
	for i := 0; i < count; i++ {
		size := (len(array) - 1) / (count - i) + 1;

		splits[i] = array[:size]
		array = array[size:]
	}
	return splits
}