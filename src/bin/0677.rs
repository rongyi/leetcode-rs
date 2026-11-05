struct Solution;

use std::collections::HashMap;

struct MapSum {
    words: HashMap<String, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MapSum {
    fn new() -> Self {
        Self {
            words: HashMap::new(),
        }
    }

    fn insert(&mut self, key: String, val: i32) {
        self.words.insert(key, val);
    }

    fn sum(&self, prefix: String) -> i32 {
        self.words
            .iter()
            .filter(|kv| kv.0.len() >= prefix.len() && kv.0.starts_with(&prefix))
            .fold(0, |mut acc, kv| {
                acc += *kv.1;
                acc
            })
    }
}

fn main() {}
