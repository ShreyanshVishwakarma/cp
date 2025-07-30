//armstrong number

fn is_armstrong(mut n: u32) -> bool {
    let num = n;
    let mut cube_sum = 0;
    while n != 0 {
        let digit = n % 10;
        cube_sum += digit.pow(3);
        n /= 10;
    }

    if cube_sum == num {
        println!("true");
        return true;
    }
    println!("false");
    false
}

fn main() {
    let num = 153;
    is_armstrong(num);
}
