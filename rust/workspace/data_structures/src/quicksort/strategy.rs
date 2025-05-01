pub fn quicksort_last<T: Ord>(arr: &mut [T]) {
    quicksort_strategy(arr, partition_last);
}

pub fn quicksort_median_of_three<T: Ord>(arr: &mut [T]) {
    quicksort_strategy(arr, median_of_three);
}

pub fn quicksort_random<T: Ord>(arr: &mut [T]) {
    quicksort_strategy(arr, partition_random);
}

const INSERTION_SORT_THRESHOLD: usize = 16;

pub fn quicksort_hybrid<T: Ord>(arr: &mut [T]) {
    quicksort_hybrid_strategy(arr, median_of_three);
}

fn quicksort_hybrid_strategy<T: Ord>(arr: &mut [T], strategy: fn(&mut [T]) -> usize) {
    if arr.len() <= INSERTION_SORT_THRESHOLD {
        crate::insertion_sort::insertion_sort(arr);
        return;
    }

    let p = strategy(arr);
    let (left, right) = arr.split_at_mut(p);
    quicksort_hybrid_strategy(left, strategy);
    quicksort_hybrid_strategy(&mut right[1..], strategy);
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

fn median_of_three<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let mid = len / 2;
    let high = len - 1;

    let mut median = high;
    if (arr[mid] >= arr[0] && arr[mid] <= arr[high])
        || (arr[mid] >= arr[high] && arr[mid] <= arr[0])
    {
        median = mid;
    } else if (arr[0] >= arr[mid] && arr[0] <= arr[high])
        || (arr[0] >= arr[high] && arr[0] <= arr[mid])
    {
        median = 0;
    }

    arr.swap(median, high);
    partition_last(arr)
}

fn partition_random<T: Ord>(arr: &mut [T]) -> usize {
    use rand::Rng;
    let mut rng = rand::rng();
    let pivot = rng.random_range(0..arr.len());
    arr.swap(pivot, arr.len() - 1);
    partition_last(arr)
}
