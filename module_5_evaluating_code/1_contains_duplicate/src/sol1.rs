use std::time::{Duration, Instant};

use sorting_rs::{bubble_sort, merge_sort};

mod helpers;

fn contains_duplicate(mut numbers: Vec<i32>) -> bool {
    numbers.sort();
    //bubble_sort(&mut numbers);
    //merge_sort(&mut numbers);
    
    for i in 0..numbers.len() - 1 {
        if numbers[i] == numbers[i+1] {
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
