use malloc::MALLOC;
use fast_vec::FastVec;
use tracker::Tracker;

#[test]
fn empty_memory() {
    MALLOC.clear();

    let v: FastVec<i32> = FastVec::new();
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 1);
    assert_eq!(MALLOC.state().allocations().len(), 1);
    assert_eq!(MALLOC.state().transcript().len(), 1);
    assert!(MALLOC.state().transcript()[0].is_allocation(size_of::<i32>()));

    MALLOC.clear();
}

#[test]
fn push_memory() {
    MALLOC.clear();

    let mut v = FastVec::new();
    assert_eq!(MALLOC.state().allocations().len(), 1);
    assert_eq!(MALLOC.state().transcript().len(), 1);
    assert!(MALLOC.state().transcript()[0].is_allocation(size_of::<String>()));


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
    assert!(MALLOC.state().transcript()[3].is_allocation(size_of::<String>() * 4));
    assert!(MALLOC.state().transcript()[4].is_free());

    v.clear();
    assert_eq!(MALLOC.state().allocations().len(),  0);
    assert_eq!(MALLOC.state().transcript().len(), 6);
    assert!(MALLOC.state().transcript()[5].is_free());

    MALLOC.clear();
}

#[test]
fn remove_memory() {
    MALLOC.clear();

    // Set up the vectors.
    let input = vec![-1, 3, -200, 25, 33];
    let mut v = FastVec::new();
    for num in input {
        v.push(num);
    }

    assert_eq!(MALLOC.state().allocations().len(), 1);
    assert_eq!(MALLOC.state().transcript().len(), 7);

    // Indices to remove in order.
    let removes = vec![1, 3, 0, 1, 0];
    for i in 0..removes.len() - 1 {
        let remove = removes[i];
        v.remove(remove);

        assert_eq!(MALLOC.state().allocations().len(), 1);
        assert_eq!(MALLOC.state().transcript().len(), 7);
    }
    v.remove(*removes.last().unwrap());

    assert_eq!(MALLOC.state().allocations().len(), 1);
    assert_eq!(MALLOC.state().transcript().len(), 7);

    MALLOC.clear();
}

#[test]
fn push_tracker() {
    MALLOC.clear();

    let mut tracker = Tracker::new();
    let mut v = FastVec::new();

    v.push(tracker.track(String::from("hello")));
    v.push(tracker.track(String::from("bye")));
    v.push(tracker.track(String::from("morning")));

    assert_eq!(v.len(), 3);
    assert_eq!(v.capacity(), 4);
    assert_eq!(tracker.tracked_count(), 3);

    drop(v);

    MALLOC.clear();
}

#[test]
fn remove_tracker() {
    MALLOC.clear();

    let mut tracker = Tracker::new();
    let mut v = FastVec::from_vec(
        vec![
            tracker.track(String::from("hello")),
            tracker.track(String::from("bye")),
            tracker.track(String::from("morning")),
            tracker.track(String::from("hello again")),
        ]
    );

    assert_eq!(v.len(), 4);
    assert_eq!(v.capacity(), 4);
    assert_eq!(tracker.tracked_count(), 4);

    v.remove(1);
    assert_eq!(tracker.tracked_count(), 3);
    v.remove(2);
    assert_eq!(tracker.tracked_count(), 2);
    v.remove(0);
    assert_eq!(tracker.tracked_count(), 1);
    v.remove(0);
    assert_eq!(tracker.tracked_count(), 0);

    drop(v);

    MALLOC.clear();
}


#[test]
fn clear_tracker() {
    MALLOC.clear();

    let mut tracker = Tracker::new();
    let mut v = FastVec::from_vec(
        vec![
            tracker.track(String::from("hello")),
            tracker.track(String::from("bye")),
            tracker.track(String::from("morning")),
            tracker.track(String::from("hello again")),
        ]
    );

    assert_eq!(v.len(), 4);
    assert_eq!(v.capacity(), 4);
    assert_eq!(tracker.tracked_count(), 4);

    v.clear();
    assert!(tracker.is_empty());

    drop(v);

    MALLOC.clear();
}