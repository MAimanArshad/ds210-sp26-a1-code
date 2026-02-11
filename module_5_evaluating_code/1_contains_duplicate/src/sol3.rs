use std::collections::HashSet;
use std::time::{Duration, Instant};

mod helpers;

fn contains_duplicate(numbers: Vec<i32>) -> bool {
    let mut set = HashSet::new();
    
    for num in numbers {
        if set.contains(&num) {
            return true;
        }
        set.insert(num);
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
