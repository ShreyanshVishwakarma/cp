// basic maths.rs
//
fn gcd(x: i32, y: i32) -> i32 {
    if x == 0 {
        return y;
    }
    if y == 0 {
        return x;
    }

    if x > y {
        if x % y == 0 {
            return y;
        }
        gcd(x - y, y)
    } else {
        if y % x == 0 {
            return x;
        }
        gcd(x, y - x)
    }
}

fn main() {
    let and = gcd(8, 16);
    print!("{and}");
}
