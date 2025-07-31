fn recursive_bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    bubble_sort(arr, arr.len());
}

fn bubble_sort<T: PartialOrd>(arr: &mut [T], n: usize) {
    if n <= 1 {
        return;
    }

    for i in 0..(n - 1) {
        if arr[i] > arr[i + 1] {
            arr.swap(i, i + 1);
        }
    }
    bubble_sort(arr, n - 1);
}

fn main() {
    let mut arr: Vec<i32> = vec![2, 5, 7, 9, 0, 8, 6, 4, 2, 1];
    println!("array before soerting : {arr:?}");
    recursive_bubble_sort(&mut arr);
    println!("array before soerting : {arr:?}");
}
