use std::collections::HashMap;
use malloc::MALLOC;
use malloc::state::Operation;

#[test]
fn test_state_transcript() {
    // Clear allocations.
    MALLOC.clear();

    // We will keep track of state here.
    let mut transcript = Vec::new();
    let mut allocations = HashMap::new();

    // Make first allocation.
    let ptr1 = MALLOC.malloc(8);
    assert!(!ptr1.is_null());

    // Test first allocation.
    transcript.push(Operation::Allocation(ptr1 as usize, 8));
    allocations.insert(ptr1 as usize, 8);
    assert_eq!(MALLOC.state().transcript(), &transcript);
    assert_eq!(MALLOC.state().allocations(), &allocations);

    // Make second allocation.
    let ptr2 = MALLOC.malloc(4);
    assert!(!ptr2.is_null());

    // Test second allocation.
    transcript.push(Operation::Allocation(ptr2 as usize, 4));
    allocations.insert(ptr2 as usize, 4);
    assert_eq!(MALLOC.state().transcript(), &transcript);
    assert_eq!(MALLOC.state().allocations(), &allocations);

    // Free.
    MALLOC.free(ptr2);

    // Test free.
    transcript.push(Operation::Free(ptr2 as usize));
    allocations.remove(&(ptr2 as usize)).unwrap();
    assert_eq!(MALLOC.state().transcript(), &transcript);
    assert_eq!(MALLOC.state().allocations(), &allocations);

    // Make some more operations.
    let ptr3 = MALLOC.malloc(4);
    MALLOC.free(ptr1);
    MALLOC.free(ptr3);

    transcript.push(Operation::Allocation(ptr3 as usize, 4));
    transcript.push(Operation::Free(ptr1 as usize));
    transcript.push(Operation::Free(ptr3 as usize));
    allocations.clear();
    assert_eq!(MALLOC.state().transcript(), &transcript);
    assert_eq!(MALLOC.state().allocations(), &allocations);
}