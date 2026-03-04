// This will create a big vector and then return a pointer to it.
fn big_vector(value: i32) -> *const Vec<i32> {
    // We create a local vector v...
    let mut v = Vec::new();

    // Fill v with a million elements,
    for _ in 0..1000000 {
        v.push(value);
    }

    // Now we want to return a pointer to it to avoid copying all the elements, what could go wrong?
    let v_ptr = &v as *const Vec<i32>;
    return v_ptr;
}

fn main() {
    let my_ptr = big_vector(10);
    let my_ptr_2 = big_vector(20);
    unsafe {
        let element_at_0 = (&*my_ptr)[0];
        println!("{element_at_0}");
    }
}
