use std::time::Instant;

mod helpers;

fn majority_element(v: &Vec<i32>) -> &i32 {
    loop {
        // println!("Guessing again");
        let random_number = helpers::random_element(v);

        let mut count_of_random_number = 0;
        for i in v {
            if random_number == i {
                count_of_random_number += 1;
            }
        }

        if count_of_random_number > v.len() / 2 {
            return random_number;
        }
    }
}

fn main() {
    let vec = helpers::random_vector(10_000_000);
    let start = Instant::now();
    let result = majority_element(&vec);
    let duration = start.elapsed();
    println!("majority_element = {result}, took {duration:?}");
}
