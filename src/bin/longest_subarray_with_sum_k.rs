use std::collections::HashMap;
//
//this only works for strictly positive and non-zero arrays
//this is better solution . not the best
fn longets_subarry_with_sum_k(arr: &[i32], k: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut vec: Vec<i32> = Vec::new();
    let mut sum: i32 = 0;
    let mut last_index = 0;
    let mut max_length: i32 = 0;
    for (i, v) in arr.iter().enumerate() {
        sum += v;
        // to handle zeros in array just dont update the map in case of same sum
        map.entry(sum).or_insert(i as i32);
        println!("key :{sum} and value : {i} added");
        if sum == k {
            max_length = i as i32 + 1;
            //this assumes there are no negative values in array , which is the case here
            last_index = i;
            continue;
        }
        let some = map.get(&(sum - k));
        if let Some(&val) = some {
            if (val - i as i32).abs() > max_length {
                max_length = (val - i as i32).abs();
                last_index = i;
                println!("hit found at index {last_index}");
            }
        }
    }
    let start = (last_index + 1) - max_length as usize;
    for i in (start..=last_index).rev() {
        vec.push(arr[i]);
    }
    vec.reverse();
    vec
}
fn main() {
    let arr = vec![1, 4, 6, 8, 5, 1, 0, 1, -1, 1, 1, 3, 6, 8];
    let vec = longets_subarry_with_sum_k(&arr, 3);
    println!("{vec:?}");
}
