// recursion ka guyaann
fn rec(vec: &mut Vec<i32>, start: usize, end: usize) {
    if start >= end {
        return;
    }
    vec.swap(start, end);
    rec(vec, start + 1, end - 1)
}

fn main() {
    let mut vec: Vec<i32> = (0..10).collect();
    println!("value of array before reversing : {vec:?}");
    rec(&mut vec, 0, 9);
    println!("value of array after reversing : {vec:?}");
}
