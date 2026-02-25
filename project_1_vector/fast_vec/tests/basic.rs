use rand::RngExt;
use fast_vec::FastVec;

#[test]
fn empty() {
    let v: FastVec<i32> = FastVec::new();
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 1);
}

#[test]
fn non_empty() {
    let mut v: FastVec<i32> = FastVec::new();
    v.push(10);
    assert_eq!(v.len(), 1);
    assert_eq!(v.capacity(), 1);
}

#[test]
fn get_strings() {
    let inputs = vec![
        String::from("hello"),
        String::from("bye"),
        String::from("morning"),
    ];
    let fast_vec = FastVec::from_vec(inputs);
    assert_eq!(fast_vec.get(1), "bye");
    assert_eq!(fast_vec.get(0), "hello");
    assert_eq!(fast_vec.get(2), "morning");
}

#[test]
fn push_and_read_numbers() {
    let mut v = FastVec::new();
    v.push(1);
    assert_eq!(v.len(), 1);
    assert_eq!(v.capacity(), 1);
    v.push(33);
    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 2);
    v.push(-5);
    assert_eq!(v.len(), 3);
    assert_eq!(v.capacity(), 4);
    assert_eq!(v.into_vec(), vec![1, 33, -5]);
}

#[test]
fn into_vec_numbers() {
    let mut rng = rand::rng();
    let mut input = Vec::with_capacity(100);
    for _ in 0..100 {
        input.push(rng.random_range(0..50));
    }

    let mut v = FastVec::new();
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

    let mut v = FastVec::new();
    for string in &input {
        v.push(string.to_owned());
    }

    assert_eq!(input, v.into_vec());
}

#[test]
fn remove_numbers() {
    // Set up the vectors.
    let mut input = vec![-1, 3, -200, 25, 33];
    let mut v = FastVec::from_vec(input.clone());
    assert_eq!(v.capacity(), 5);

    // Indices to remove in order.
    let removes = vec![1, 3, 0, 1, 0];
    for remove in removes {
        v.remove(remove);
        input.remove(remove);

        assert_eq!(v.len(), input.len());
        assert_eq!(v.capacity(), 5);
        for i in 0..input.len() {
            assert_eq!(v.get(i), &input[i]);
        }
    }

    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 5);
}