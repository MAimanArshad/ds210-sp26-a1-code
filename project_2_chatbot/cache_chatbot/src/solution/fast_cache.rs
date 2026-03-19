// This is a correct and fast implementation of a 
// Least recently used (LRU) cache.
// See the docs if you are curious: https://docs.rs/lru/latest/lru/

use std::num::NonZero;
use lru::LruCache;

pub struct Cache<V> {
    lru: LruCache<String, V>,
}

impl<V> Cache<V> {
    pub fn new(max_size: usize) -> Cache<V> {
        let max_size = NonZero::new(max_size).unwrap();
        return Cache {
            lru: LruCache::new(max_size),
        };
    }
    pub fn get_chat(&mut self, username: &str) -> Option<&mut V> {
        return self.lru.get_mut(username);
    }

    pub fn insert_chat(&mut self, username: String, chat: V) {
        self.lru.put(username, chat);
    }
}