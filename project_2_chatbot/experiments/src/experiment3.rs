extern crate experiments;

use std::collections::HashMap;
use std::time::Instant;

use cache_chatbot::solution::slow_cache::Cache as SlowCache;
use cache_chatbot::solution::fast_cache::Cache as FastCache;

const CAPACITY: usize = 10000000;

fn main() {
    // SlowCache
    println!("Filling up slow cache with {} elements", CAPACITY);
    let mut map = HashMap::new();
    let mut history = Vec::new();
    for i in 0..CAPACITY {
        let username = format!("u{}", i);
        let msg = format!("{}", i);
        map.insert(username.clone(), msg);
        history.push(username);        
    }
    let mut cache = SlowCache::prime(CAPACITY, map, history);

    // Access an element.
    let now = Instant::now();
    let msg = cache.get_chat(&"u0");
    let slow_access = now.elapsed();
    if msg.unwrap() != "0" {
        panic!("slow cache does not access elements correctly");
    }
    
    // Insert an element.
    let now = Instant::now();
    cache.insert_chat(String::from("x0"), String::from("xx"));
    let slow_insert = now.elapsed();
    if cache.get_chat("x0").unwrap() != "xx" {
        panic!("slow cache does not insert elements correctly");
    }
    

    // FastCache
    let mut cache = FastCache::new(CAPACITY);
    println!("Filling up fast cache with {} elements", CAPACITY);
    for i in 0..CAPACITY {
        let username = format!("u{}", i);
        let msg = format!("{}", i);
        cache.insert_chat(username, msg);
    }

    // Access an element.
    let now = Instant::now();
    let msg = cache.get_chat(&"u0");
    let fast_access = now.elapsed();
    if msg.unwrap() != "0" {
        panic!("fast cache does not access elements correctly");
    }
    
    // Insert an element.
    let now = Instant::now();
    cache.insert_chat(String::from("x0"), String::from("xx"));
    let fast_insert = now.elapsed();
    if cache.get_chat("x0").unwrap() != "xx" {
        panic!("fast cache does not insert elements correctly");
    }

    // Print experiment results
    println!("----- Experiment Results -----");
    println!(" SlowCache:");
    println!("  access: {:?}", slow_access);
    println!("  insert: {:?}", slow_insert);
    println!(" FastCache:");
    println!("  access: {:?}", fast_access);
    println!("  insert: {:?}", fast_insert);
    println!("----- End of Experiment Results -----");
}