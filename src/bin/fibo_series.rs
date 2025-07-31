fn nth_fibo(n: i32, map: &mut std::collections::HashMap<i32, usize>) -> usize {
    if n == 1 {
        return 0;
    }
    if n == 2 {
        return 1;
    }
    if map.contains_key(&n) {
        return map[&n];
    };
    let result = nth_fibo(n - 1, map) + nth_fibo(n - 2, map);
    map.insert(n, result);
    result
}

fn main() {
    let mut map: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();
    for i in 1..100 {
        let result = nth_fibo(i, &mut map);
        println!(" the {i}th fibo number is : {result:?}");
    }
}
