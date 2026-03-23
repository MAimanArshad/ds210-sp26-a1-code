use cache_chatbot::solution::slow_cache::Cache as SlowCache;

#[test]
fn slow_cache() {
    let mut cache = SlowCache::new(5);

    // Prime with 5.
    for i in 0..5 {
        let username: String = format!("u{}", i);
        let mut msg = format!("{}", i);
        cache.insert_chat(username.clone(), msg.clone());
        assert_eq!(cache.get_chat(&username), Some(&mut msg));
    }

    // Access the first two elements: they are now most recently used.
    assert_eq!(cache.get_chat("u0").unwrap(), "0");
    assert_eq!(cache.get_chat("u1").unwrap(), "1");

    // insert 3 new elements.
    for i in 0..3 {
        let evicted = format!("u{}", i + 2);
        let username: String = format!("u{}", i + 10);
        let mut msg = format!("{}", i + 10);

        cache.insert_chat(username.clone(), msg.clone());

        println!("Inserted {} -- should have evicted {}", username, evicted);
        assert_eq!(cache.get_chat(&evicted), None);
        assert_eq!(cache.get_chat(&username), Some(&mut msg));
    }

    // First two elements are still there.
    assert_eq!(cache.get_chat("u0").unwrap(), "0");
    assert_eq!(cache.get_chat("u1").unwrap(), "1");
}
