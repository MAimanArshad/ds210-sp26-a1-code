fn main() {
    // Initially, ptr points to address 0.
    let mut ptr = 0 as *const String;
    // or alternatively,
    // let mut ptr = std::ptr::null();

    // TODO: try with my_var = 1, 2, and 3.
    let my_var = 1;

    let x1 = String::from("Hello");
    if my_var < 2 {
        let x2 = String::from("Goodbye");
        if my_var == 1 {
            ptr = &x2 as *const String;
        }
    } else if my_var == 2 {
        ptr = &x1 as *const String;
    }

    unsafe {
        println!("The address that ptr points to is {:p}", ptr);
        println!("The value that ptr points to is {}", *ptr);
    }
}
