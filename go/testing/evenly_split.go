package hzw

func EvenlySplitInt64(array []int64, batchSize int) [][]int64 {
	splits := [][]int64{}

	if len(array) <= batchSize {
		if len(array) == 0 {
			return splits
		}
		return append(splits, array)
	}

	bs := batchSize
	count := 0
	remain := 0
	for {
		count = len(array) / bs
		remain = len(array) % bs
		if remain == 0 {
			break
		} else if remain == bs - 1 {
			if bs != batchSize && remain == 1 {
				break
			}
			count++
			remain = 0
			break
		} else {
			bs--
		}
	}

	for i := 0; i < count; i++ {
		size := bs
		if size > len(array) {
			size = len(array)
		}
		if remain > 0 {
			size++
		}
		remain--
		splits = append(splits, array[:size])
		array = array[size:]
	}
	
	return splits
}