use std::{collections::HashMap, fmt::Debug};

pub struct Cache<V> {
    max_size: usize,
    hashmap: HashMap<String, V>,
    usage_history: Vec<String>,
}

impl<V> Cache<V> {
    // We use this function for testing,
    // Do not use it or modify it.
    pub fn prime(
        max_size: usize,
        hashmap: HashMap<String, V>,
        usage_history: Vec<String>
    ) -> Cache<V> {
        return Cache {
            max_size,
            hashmap,
            usage_history,
        };
    }

    // Create a new Cache with the given capacity.
    pub fn new(max_size: usize) -> Cache<V> {
        return Cache {
            max_size,
            hashmap: HashMap::new(),
            usage_history: Vec::new(),
        }
    }

    // Helper functions.
    fn remove_least_recently_used(&mut self) {
    if self.usage_history.is_empty() {
        return;
        }
    let least_recently_used = self.usage_history.remove(0);
    self.hashmap.remove(&least_recently_used);
    println!("Removing least recently used");
    }

    }
    fn mark_as_most_recently_used(&mut self, username: String) {
        if self.usage_history.contains(&username) {
            self.usage_history.retain(|x| x != &username);
        }
        self.usage_history.push(username.clone());
        println!("Marking {username} as most recently used");
    }

    // Reading from the cache:
    // if the username is in the cache, it must be marked as the most recently
    // used.
    pub fn get_chat(&mut self, username: &str) -> Option<&mut V> {
        if self.hashmap.contains_key(username) {
            self.mark_as_most_recently_used(username.to_string());
            return self.hashmap.get_mut(username);
        }
        return None;
    }

    // Inserting to the cache:
    // 1. What if cache is at capacity?
    // 2. What should be the most recently used chat after this insertion?
    pub fn insert_chat(&mut self, username: String, chat: V) {
        println!("Insert {username} into cache:");
        println!("Cache before inserting: -----");
        println!("{:?}", self);
        println!("-----------------------------");

        self.hashmap.insert(username.clone(), chat);
        self.mark_as_most_recently_used(username);
        if self.hashmap.len() > self.max_size {
            self.remove_least_recently_used();
        }

        println!("Cache after inserting: ------");
        println!("{:?}", self);
        println!("-----------------------------");
    }
}

// This allows you to print the cache and view its state.
impl<V> Debug for Cache<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let map: HashMap<_, _> = self.hashmap.iter()
            .map(|(k, v)| (k, "<chat>"))
            .collect();
        write!(f, "Cache {{\n  hashmap = {:?},\n  usage_history = {:?}\n}}", map, self.usage_history)
    }
}

// You can use this for testing!
pub fn main() {
    println!("Creating cache with capacity 3");
    let mut cache = Cache::new(3);
    cache.insert_chat(String::from("user1"), String::from("v1"));
    cache.insert_chat(String::from("user2"), String::from("v2"));
    cache.insert_chat(String::from("user3"), String::from("v3"));
    cache.insert_chat(String::from("user4"), String::from("v4"));

    // Read the last chat.
    let v4 = cache.get_chat("user4").unwrap();
    assert_eq!(v4, "v4");

    // Read the first chat.
    let v1 = cache.get_chat("user1");
    match v1 {
        None => {},
        Some(_) => {
            println!("Error: the implementation of slow cache is either incomplete or not correct.");
            println!("We set up the cache to have a max size = 3.");
            println!("We then inserted 4 different values into it: user1, user2, user3, and user4 in order!");
            println!("When user 4 gets inserted, a correct implementation should realize the size is now larger than 3, so the cache should remove the entry that was least recently used.");
            println!("In this case, that entry is user1.");
            println!("But user1 does not get removed.");
            return;
        }
    }

    // Read user2 to make it be not the least recently used.
    assert_eq!(cache.get_chat("user2"), Some(&mut String::from("v2")));

    // Insert a new user.
    cache.insert_chat(String::from("user5"), String::from("v5"));

    // Read user2.
    let v2 = cache.get_chat("user2");
    match v2 {
        None => {
            println!("Error: the implementation of slow cache is either not correct.");
            println!("We set up the cache to have a max size = 3.");
            println!("We performed these operations in order:");
            println!(" Insert user1");
            println!(" Insert user2");
            println!(" Insert user3");
            println!(" Insert user4");
            println!(" Read user4");
            println!(" Read user1");
            println!(" Read user2");
            println!(" Insert user5");
            println!(" Read user2 <--- this is removed by your cache, but it should not be");
            println!("Your implementation removes user2 to make space for inserting user 5.");
            println!("This is incorrect.");
            println!("Which element should it remove instead? Use a pen and paper and try to keep track of which user is the least recently used with the operations above.");
            return;
        },
        Some(v2) => {
            assert_eq!(v2, "v2");
        }
    }
}