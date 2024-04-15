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
        let mut val = 0;

        for (k, v) in self.words.iter() {
            if k.len() < prefix.len() {
                continue;
            }
            if k.starts_with(&prefix) {
                val += *v;
            }
        }

        val as i32
    }
}
fn main() {}
