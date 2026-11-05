
struct Solution;

use std::collections::{BTreeSet, HashMap};
struct NumberContainers {
    // idx -> val
    idxs: HashMap<i32, i32>,
    // val -> idx...
    vals: HashMap<i32, BTreeSet<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumberContainers {
    fn new() -> Self {
        Self {
            idxs: HashMap::new(),
            vals: HashMap::new(),
        }
    }

    fn change(&mut self, index: i32, number: i32) {
        // ok, let delete this index
        if let Some(prev_val) = self.idxs.get(&index) {
            if let Some(idx_set) = self.vals.get_mut(prev_val) {
                idx_set.remove(&index);
            }
        }
        self.idxs.insert(index, number);
        self.vals.entry(number).or_default().insert(index);
    }

    fn find(&self, number: i32) -> i32 {
        if let Some(idx_set) = self.vals.get(&number) {
            if let Some(val) = idx_set.iter().next() {
                *val
            } else {
                -1
            }
        } else {
            -1
        }
    }
}

fn main() {}
