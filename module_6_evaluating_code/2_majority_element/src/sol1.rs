use std::collections::HashMap;
use std::time::Instant;

mod helpers;

fn majority_element(v: Vec<i32>) -> i32 {
    let n = v.len();

    let mut count_map = HashMap::new();
    for number in v {
        match count_map.get(&number) {
            None => {
                count_map.insert(number, 1);
            },
            Some(count) => {
                count_map.insert(number, count + 1);
            }
        }
    }

    for (number, count) in count_map {
        if count > n / 2 {
            return number;
        }
    }

    return -1;
}

fn main() {
    let vec = helpers::random_vector(10_000_000);
    let start = Instant::now();
    let result = majority_element(vec);
    let duration = start.elapsed();
    println!("majority_element = {result}, took {duration:?}");
}
