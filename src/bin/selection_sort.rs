fn selection_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();

    for i in 0..n {
        let mut min_index = i;
        for j in (i + 1)..n {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        if min_index != i {
            arr.swap(i, min_index);
        }
    }
}

fn main() {
    let mut arr = vec![2, 5, 7, 9, 0, 1, 3, 5, 6, 8, 3, 8, 9];
    println!("array before sorting : {arr:?}");
    selection_sort(&mut arr);
    println!("array after sorting : {arr:?}");
}
