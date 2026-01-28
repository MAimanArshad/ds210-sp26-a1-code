fn parity(x: i64) -> i64 {
    return x % 2; 
}

fn add(a: i64, b: i64) -> i64{
    return a + b;
}

fn main() {
    let a = 5;
    let b = 6;
    println!("Parity of {} + {} is {}", a, b, parity(add(a, b)));
}
