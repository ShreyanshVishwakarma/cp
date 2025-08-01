fn largest_number<T: PartialOrd + Clone>(arr: &[T]) -> T {
    let mut largest = arr[0].clone();
    for i in 1..arr.len() {
        if arr[i] > largest {
            largest = arr[i].clone();
        }
    }
    largest
}

fn main() {
    let arr = vec![2, 3, 4, 5, 7, 8, 9, 2, 5, 9, 0];
    println!("largest number in array: {:?}", largest_number(&arr));
}
