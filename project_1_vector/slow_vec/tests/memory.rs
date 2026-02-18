use malloc::MALLOC;
use slow_vec::SlowVec;

#[test]
fn empty() {
    MALLOC.clear();

    let v: SlowVec<i32> = SlowVec::new();
    assert_eq!(v.len(), 0);
    assert_eq!(MALLOC.state().transcript().len(), 0);

    MALLOC.clear();
}

#[test]
fn push_and_read_strings() {
    MALLOC.clear();

    let mut v = SlowVec::new();
    assert_eq!(MALLOC.state().allocations().len(), 0);
    assert_eq!(MALLOC.state().transcript().len(), 0);

    v.push(String::from("hello"));
    assert_eq!(MALLOC.state().allocations().len(), 1);
    assert_eq!(MALLOC.state().transcript().len(), 1);
    assert!(MALLOC.state().transcript()[0].is_allocation(size_of::<String>()));

    v.push(String::from("bye"));
    assert_eq!(MALLOC.state().allocations().len(), 1);
    assert_eq!(MALLOC.state().transcript().len(), 3);
    assert!(MALLOC.state().transcript()[1].is_allocation(size_of::<String>() * 2));
    assert!(MALLOC.state().transcript()[2].is_free());

    v.push(String::from("morning"));
    assert_eq!(MALLOC.state().allocations().len(), 1);
    assert_eq!(MALLOC.state().transcript().len(), 5);
    assert!(MALLOC.state().transcript()[3].is_allocation(size_of::<String>() * 3));
    assert!(MALLOC.state().transcript()[4].is_free());

    v.clear();
    assert_eq!(MALLOC.state().allocations().len(),  0);
    assert_eq!(MALLOC.state().transcript().len(), 6);
    assert!(MALLOC.state().transcript()[5].is_free());

    MALLOC.clear();
}

#[test]
fn remove_numbers() {
    // Set up the vectors.
    let input = vec![-1, 3, -200, 25, 33];
    let mut v = SlowVec::new();
    for num in input {
        v.push(num);
    }

    assert_eq!(MALLOC.state().allocations().len(), 1);
    assert_eq!(MALLOC.state().transcript().len(), 9);

    // Indices to remove in order.
    let removes = vec![1, 3, 0, 1, 0];
    for i in 0..removes.len() - 1 {
        let remove = removes[i];
        v.remove(remove);

        assert_eq!(v.len(), 5 - i - 1);
        assert_eq!(MALLOC.state().allocations().len(), 1);
        assert_eq!(MALLOC.state().transcript().len(), 9 + 2*i + 2);
        assert!(MALLOC.state().transcript()[9 + 2*i].is_allocation(size_of::<i32>() * v.len()));
        assert!(MALLOC.state().transcript()[9 + 2*i + 1].is_free());
    }

    v.remove(*removes.last().unwrap());
    assert_eq!(MALLOC.state().allocations().len(), 0);
    assert_eq!(MALLOC.state().transcript().len(), 9 + 2 * removes.len() - 1);
    assert!(MALLOC.state().transcript()[9 + 2 * removes.len() - 2].is_free());
}

#[test]
fn clear_vec() {
    let mut v = SlowVec::from_vec(vec![1, 2, 3]);
    assert_eq!(MALLOC.state().allocations().len(), 1);
    assert_eq!(MALLOC.state().transcript().len(), 1);
    assert!(MALLOC.state().transcript()[0].is_allocation(size_of::<i32>() * 3));

    v.clear();
    assert_eq!(MALLOC.state().allocations().len(), 0);
    assert_eq!(MALLOC.state().transcript().len(), 2);
    assert!(MALLOC.state().transcript()[1].is_free());
}
