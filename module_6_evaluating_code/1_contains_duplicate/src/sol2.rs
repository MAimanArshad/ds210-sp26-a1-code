use std::collections::HashMap;
use std::time::{Duration, Instant};

mod helpers;

fn contains_duplicate(numbers: Vec<i32>) -> bool {
    let mut map = HashMap::new();
    
    for num in numbers {
        let value_in_map = map.get(&num);
        match value_in_map {
            None => {
                map.insert(num, 1);
            },
            Some(count) => {
                map.insert(num, count + 1);
            }
        }
    }

    for (num, count) in map {
        if count > 1 {
            return true;
        }
    }
    
    return false;
}



fn main() {
    let v = helpers::random_vector(10_000_000);
    println!("Generated a random vector of size {}", v.len());
    
    let now = Instant::now();    
    let result = contains_duplicate(v);
    let elapsed = now.elapsed();

    println!("Output {result} - Took {elapsed:?}");
}
