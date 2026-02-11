use std::time::Instant;

mod helpers;

fn main() {
    let numbers = 
        helpers::large_list(10000000);

    // Start timer.
    let start_time = Instant::now();

    // Sum all the numbers
    let mut sum = 0;
    for number in numbers {
        sum = sum + number;
    }

    // how much time did the sum take?
    let duration = start_time.elapsed();

    println!("Sum is {sum}, time it took is {duration:?}");
}
