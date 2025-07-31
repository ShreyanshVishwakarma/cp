fn recursive_insertion_sort<T: PartialOrd>(arr: &mut [T]) {
    //
    insertion_sort(arr, 1);
}

fn insertion_sort<T: PartialOrd>(arr: &mut [T], n: usize) {
    if n >= arr.len() {
        return;
    }
    let mut j = n;
    while j > 0 && arr[j] < arr[j - 1] {
        arr.swap(j, j - 1);
        j -= 1;
    }
    insertion_sort(arr, n + 1);
}

fn main() {
    let mut arr = vec![1, 0, 2, 9, 3, 9, 4, 8, 5, 8, 6, 7];
    println!("array before sorting = {arr:?}");
    recursive_insertion_sort(&mut arr);
    println!("array after sorting = {arr:?}");
}
