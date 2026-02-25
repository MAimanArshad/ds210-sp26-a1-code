use malloc::MALLOC;

#[test]
fn test_malloc() {
    let ptr = MALLOC.malloc(4);
    assert!(!ptr.is_null());

    let actual_ptr = ptr as *mut u32;
    unsafe {*actual_ptr = u32::MAX - 10;};
    assert_eq!(unsafe { *actual_ptr }, u32::MAX - 10);

    MALLOC.free(ptr);
}

#[test]
fn test_malloc2() {
    let ptr = MALLOC.malloc(0);
    assert!(!ptr.is_null());
    MALLOC.free(ptr);
}