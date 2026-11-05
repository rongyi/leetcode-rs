#![allow(dead_code)]

struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut cnt: HashMap<i32, i32> = HashMap::new();
        for &num in arr.iter() {
            *cnt.entry(num).or_insert(0) += 1;
        }

        let cnt_set: HashSet<i32> = cnt.values().cloned().collect();
        cnt_set.len() == cnt.values().len()
    }
}

fn main() {}
