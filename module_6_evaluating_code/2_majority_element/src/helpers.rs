use rand::{rng, RngExt};
use rand::seq::SliceRandom;

pub fn random_vector(size: i32) -> Vec<i32> {
    let mut vec: Vec<i32> = (0..(size - 1) / 2).collect();
    while vec.len() < size as usize {
        vec.push(10);
    }
    vec.shuffle(&mut rng());
    return vec;
}


pub fn random_element<T>(v: &Vec<T>) -> &T {
    let mut rng = rng();
    let index = rng.random_range(0..v.len());
    return &v[index];
}