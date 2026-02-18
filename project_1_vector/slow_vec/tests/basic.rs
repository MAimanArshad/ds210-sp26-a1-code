use rand::RngExt;
use slow_vec::SlowVec;

#[test]
fn empty() {
    let v: SlowVec<i32> = SlowVec::new();
    assert_eq!(v.len(), 0);
}

#[test]
fn non_empty() {
    let mut v: SlowVec<i32> = SlowVec::new();
    v.push(10);
    assert_eq!(v.len(), 1);
}

#[test]
fn push_and_read_numbers() {
    let mut v = SlowVec::new();
    v.push(1);
    v.push(33);
    v.push(-5);
    assert_eq!(v.get(0), &1);
    assert_eq!(v.get(2), &-5);
    assert_eq!(v.get(1), &33);
}

#[test]
fn push_and_read_strings() {
    let mut v = SlowVec::new();
    v.push(String::from("hello"));
    v.push(String::from("bye"));
    v.push(String::from("morning"));
    assert_eq!(v.get(1), "bye");
    assert_eq!(v.get(0), "hello");
    assert_eq!(v.get(2), "morning");
}

#[test]
fn into_vec_numbers() {
    let mut rng = rand::rng();
    let mut input = Vec::with_capacity(100);
    for _ in 0..100 {
        input.push(rng.random_range(0..50));
    }

    let mut v = SlowVec::new();
    for number in &input {
        v.push(*number);
    }

    assert_eq!(input, v.into_vec());
}

#[test]
fn into_vec_strings() {
    let mut rng = rand::rng();
    let mut input = Vec::with_capacity(100);
    for _ in 0..100 {
        let random_string = format!("str{}", rng.random_range(0..50));
        input.push(random_string);
    }

    let mut v = SlowVec::new();
    for string in &input {
        v.push(string.to_owned());
    }

    assert_eq!(input, v.into_vec());
}

#[test]
fn remove_numbers() {
    // Set up the vectors.
    let mut input = vec![-1, 3, -200, 25, 33];
    let mut v = SlowVec::new();
    for num in &input {
        v.push(*num);
    }

    // Indices to remove in order.
    let removes = vec![1, 3, 0, 1, 0];
    for remove in removes {
        v.remove(remove);
        input.remove(remove);

        assert_eq!(v.len(), input.len());
        for i in 0..input.len() {
            assert_eq!(v.get(i), &input[i]);
        }
    }

    assert_eq!(v.len(), 0);
}

#[test]
fn clear() {
    let mut v = SlowVec::from_vec(vec![1, 2, 3]);
    v.clear();
    assert_eq!(v.len(), 0);
}