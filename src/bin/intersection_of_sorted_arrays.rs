use std::collections::BTreeSet;

fn intersection<T: Ord + Clone>(arr1: &[T], arr2: &[T]) -> Vec<T> {
    let set: BTreeSet<T> = arr1.iter().cloned().collect();
    let mut final_set: BTreeSet<T> = BTreeSet::new();
    for i in arr2.iter() {
        if set.contains(i) {
            final_set.insert(i.clone());
        }
    }
    final_set.into_iter().collect()
}
fn main() {
    // intersection of two arrays

    let arr1 = vec![1, 2, 3, 4, 4, 5, 5, 6, 7];
    let arr2 = vec![1, 1, 2, 2, 3, 4, 6, 7, 8, 9, 9];

    let inter = intersection(&arr1, &arr2);

    println!("the uion of arrs : arr1: {arr1:?} and arr2: {arr2:?} is {inter:?}");
}
