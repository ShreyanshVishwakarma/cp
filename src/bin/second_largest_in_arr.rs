fn second_largest<T: PartialOrd + Clone>(arr: &[T]) -> T {
    let mut largest = arr[0].clone();
    let mut second: T = arr[0].clone();
    for item in arr.iter().skip(1) {
        if item > &largest {
            second = largest;
            largest = item.clone();
        }
    }
    second
}

fn main() {
    let arr = vec![2, 3, 4, 5, 7, 8, 9, 2, 5, 9, 0];
    println!("largest number in array: {:?}", second_largest(&arr));
}
