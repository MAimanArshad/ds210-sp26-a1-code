use std::time::Instant;

mod helpers;

fn main() {
    let rows = 10000;
    let cols = 10000;
    let matrix = helpers::large_matrix(rows, cols);

    // Start timer.
    let start_time = Instant::now();

    // Sum all the numbers
    let mut sum = 0;
    for c in 0..cols {
        for r in 0..rows {
            sum = sum + matrix[r][c];
        }
    }

    // how much time did the sum take?
    let duration = start_time.elapsed();

    println!("Sum is {sum}, time it took is {duration:?}");
}
