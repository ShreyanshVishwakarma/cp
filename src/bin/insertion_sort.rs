fn insertion_sort<T: Ord>(arr: &mut [T]) {
    //someting
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn main() {
    let mut arr = vec![1, 0, 2, 9, 3, 9, 4, 8, 5, 8, 6, 7];
    println!("array before sorting = {arr:?}");
    insertion_sort(&mut arr);
    println!("array after sorting = {arr:?}");
}
