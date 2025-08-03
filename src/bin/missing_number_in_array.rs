use std::collections::HashSet;

fn missing_number(nums: Vec<i32>) -> i32 {
    let set: HashSet<i32> = nums.clone().into_iter().collect();
    for i in 0..(nums.len() + 1) {
        if set.contains(&(i as i32)) {
            continue;
        } else {
            return i as i32;
        }
    }
    unreachable!()
}

fn main() {
    let vec: Vec<i32> = (0..9).collect();
    let num = missing_number(vec);
    println!("{num}");
}
