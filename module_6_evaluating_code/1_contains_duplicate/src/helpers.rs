use rand::rng;
use rand::seq::SliceRandom;

pub fn random_vector(size: i32) -> Vec<i32> {
    let mut vec: Vec<i32> = (0..size).collect();
    vec.shuffle(&mut rng());
    return vec;
}
