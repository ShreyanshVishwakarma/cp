fn calc_freq(n: String, map: &mut std::collections::HashMap<char, i32>) {
    let arr: Vec<char> = n.chars().collect();
    for i in arr {
        *map.entry(i).or_insert(0) += 1;
    }
}

fn main() {
    let mut map: std::collections::HashMap<char, i32> = std::collections::HashMap::new();
    let s = String::from("hello wrold");
    calc_freq(s, &mut map);
    println!("{map:?}");
}
