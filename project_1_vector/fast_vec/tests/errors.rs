use fast_vec::FastVec;

#[test]
#[should_panic(expected = "FastVec: get out of bounds")]
fn read_out_of_bounds() {
    let mut v = FastVec::new();
    v.push(1);
    v.push(33);
    v.push(-5);
    v.get(3);
}

#[test]
#[should_panic(expected = "FastVec: remove out of bounds")]
fn remove_out_of_bounds() {
    let mut v = FastVec::new();
    v.push(1);
    v.push(33);
    v.push(-5);
    v.remove(3);
}