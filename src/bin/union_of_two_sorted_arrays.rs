use std::vec;

fn union<T: PartialOrd + PartialEq + Clone>(arr1: &[T], arr2: &[T]) -> Vec<T> {
    //
    let mut ptr1: usize = 0;
    let mut ptr2: usize = 0;

    let mut temp: Vec<T> = Vec::new();
    while ptr1 < arr1.len() && ptr2 < arr2.len() {
        if arr1[ptr1] <= arr2[ptr2] {
            if temp.is_empty() || temp.last().unwrap() != &arr1[ptr1] {
                temp.push(arr1[ptr1].clone());
            }
            ptr1 += 1;
        } else {
            if temp.is_empty() || temp.last().unwrap() != &arr2[ptr2] {
                temp.push(arr2[ptr2].clone());
            }
            ptr2 += 1;
        }
    }
    if ptr1 < arr1.len() {
        for _ in ptr1..arr1.len() {
            if temp.last().unwrap() != &arr1[ptr1] {
                temp.push(arr1[ptr1].clone());
            }
            ptr1 += 1;
        }
    }
    if ptr2 < arr2.len() {
        for _ in ptr2..arr2.len() {
            if temp.last().unwrap() != &arr2[ptr2] {
                temp.push(arr2[ptr2].clone());
            }
            ptr2 += 1;
        }
    }
    temp
}

fn main() {
    let arr1 = vec![1, 2, 3, 4, 4, 5, 5, 6, 7];
    let arr2 = vec![1, 1, 2, 2, 3, 4, 6, 7, 8, 9, 9];

    let unionArr = union(&arr1, &arr2);
    println!("the uion of arrs : arr1: {arr1:?} and arr2: {arr2:?} is {unionArr:?}");
}
