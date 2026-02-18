use std::time::Instant;

mod helpers;

fn majority_element(mut v: Vec<i32>) -> i32 {
    v.sort();
    return v[v.len() / 2];
}

fn main() {
    let vec = helpers::random_vector(10_000_000);
    let start = Instant::now();
    let result = majority_element(vec);
    let duration = start.elapsed();
    println!("majority_element = {result}, took {duration:?}");
}
