pub fn large_list(n: usize) -> Vec<i32> {
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        v.push((i as i32 % 9) - 4);
    }
    v
}
