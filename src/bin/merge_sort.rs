fn merge<T: PartialOrd + Clone>(arr: &mut [T], low: usize, mid: usize, high: usize) {
    let mut temp: Vec<T> = Vec::with_capacity(high - low + 1);
    let (mut left, mut right) = (low, mid + 1);

    while left <= mid && right <= high {
        if arr[left] <= arr[right] {
            temp.push(arr[left].clone());
            left += 1;
        } else {
            temp.push(arr[right].clone());
            right += 1;
        }
    }
    if left <= mid {
        temp.extend_from_slice(&arr[left..mid + 1])
    }
    if right <= high {
        temp.extend_from_slice(&arr[right..high + 1])
    }
    for (i, v) in temp.into_iter().enumerate() {
        arr[low + i] = v;
    }
}
fn merge_sort<T: PartialOrd + Clone>(arr: &mut [T], low: usize, high: usize) {
    if low >= high {
        return;
    }
    let mid = low + (high - low) / 2;
    merge_sort(arr, low, mid);
    merge_sort(arr, mid + 1, high);
    merge(arr, low, mid, high);
}

fn main() {
    let mut arr = vec![2, 4, 6, 7, 8, 9, 0, 1, 3, 5, 6];
    println!("array before sorting : {arr:?}");
    let n = arr.len();
    if n > 0 {
        merge_sort(&mut arr, 0, n - 1);
    }
    println!("array after sorting : {arr:?}");
}
