use std::collections::{BTreeMap, HashMap};

// struct Solution;
struct TimeMap {
    cache: HashMap<String, BTreeMap<i32, String>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.cache.entry(key).or_default().insert(timestamp, value);
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(ts) = self.cache.get(&key) {
            let cur = ts.range(..=timestamp).rev().next();
            match cur {
                Some((_, val)) => val.clone(),
                None => "".to_string(),
            }
        } else {
            "".to_string()
        }
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */

fn main() {}
