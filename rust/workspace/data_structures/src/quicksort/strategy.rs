pub fn quicksort_last<T: Ord>(arr: &mut [T]) {
    quicksort_strategy(arr, partition_last);
}

fn quicksort_strategy<T: Ord>(arr: &mut [T], strategy: fn(&mut [T]) -> usize) {
    if arr.len() <= 1 {
        return;
    }

    let p = strategy(arr);
    let (left, right) = arr.split_at_mut(p);
    quicksort_strategy(left, strategy);
    quicksort_strategy(&mut right[1..], strategy);
}

fn partition_last<T: Ord>(arr: &mut [T]) -> usize {
    let high = arr.len() - 1;
    let mut i = 0;
    for j in 0..high {
        if arr[j] > arr[high] {
            continue;
        }
        arr.swap(i, j);
        i += 1;
    }
    arr.swap(i, high);
    i
}
