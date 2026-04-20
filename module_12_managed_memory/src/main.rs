fn ptr() {
    let x = String::from("hello");
    let ptr = &x as *const String;
    drop(x);

    unsafe {
        println!("{}", *ptr);
    }
}

fn main() {
    ptr();
}

/*
fn reference() {
    let x = String::from("hello");
    let ref_to_x = &x;
    drop(x);
    println!("{}", ref_to_x);
}
*/