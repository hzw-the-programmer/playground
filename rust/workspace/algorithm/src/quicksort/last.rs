pub fn quicksort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let p = partition(arr);
    let (left, right) = arr.split_at_mut(p);
    quicksort(left);
    quicksort(&mut right[1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
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
