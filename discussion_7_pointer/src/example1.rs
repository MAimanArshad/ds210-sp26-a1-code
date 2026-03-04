use std::{ops::Index, time::Instant};

fn mid_by_pointer(v: *const Vec<i32>) -> i32 {
    unsafe {
        // These comments are extra information if you want to know why we write it like this.

        // * dereference the raw pointer *const Vec<i32> -> Vec<i32>. 
        // & take the reference of Vec<i32> -> &Vec<i32>

        // Note that rust will auto-deref (*v).len() to (&*v).len()
        let length = (*v).len(); // equivalent to (&*v).len()

        // But this is NOT allowed:
        return (*v)[length / 2];
        // because (*v)[length / 2] is actually (*v).index(length / 2)
        // and index() is a "trait" which Rust will not auto-deref for you
    }
}

fn mid_by_copy(v: Vec<i32>) -> i32 {
    let length = v.len();
    return v[length / 2];
}

fn main() {
    let mut vec = Vec::new();
    for i in 0..200000000 {
        vec.push(i);
    }

    // By pointer.
    let start_time1 = Instant::now();
    let mid1 = mid_by_pointer(&vec as *const Vec<i32>);
    let time1 = start_time1.elapsed();
    println!("By pointer returned {} and took {:?}", mid1, time1);

    // By copy.
    let start_time2 = Instant::now();
    let mid2 = mid_by_copy(vec.clone());
    let time2 = start_time2.elapsed();
    println!("By copy returned {} and took {:?}", mid2, time2);

    // We can can continue to use the vector afterwards to do other things.
    println!("The first and last elements in the vector are {} and {}", vec[0], vec[vec.len() - 1]);
}
