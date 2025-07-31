fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for _ in 0..(arr.len() - 1) {
        let mut is_sorted: bool = true;
        for i in 0..(arr.len() - 1) {
            if arr[i] < arr[i + 1] {
                arr.swap(i, i + 1);
                is_sorted = false;
            }
        }
        if is_sorted {
            return;
        }
    }
}

fn main() {
    let mut arr = vec![2, 5, 6, 8, 32, 5, 7, 9, 1, 0, 8];
    println!("array before sorting : {arr:?}");
    bubble_sort(&mut arr);
    println!("array after sorting : {arr:?}");
}
