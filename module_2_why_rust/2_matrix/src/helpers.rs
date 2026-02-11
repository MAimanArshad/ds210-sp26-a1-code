pub fn large_matrix(rows: usize, cols: usize) -> Vec<Vec<i32>> {
    let mut v = Vec::with_capacity(rows);
    for i in 0..rows {
        let mut r = Vec::with_capacity(cols);
        for c in 0..cols {
            r.push(((i + c) as i32 % 9) - 4);
        }
        v.push(r);
    }
    v
}
