#![allow(dead_code)]


struct Solution;

use std::collections::BTreeMap;
impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut freq: BTreeMap<i32, i32> = BTreeMap::new();
        for num in arr.into_iter() {
            *freq.entry(num).or_insert(0) += 1;
        }
        for (k, v) in freq.into_iter().rev() {
            if k == v {
                return k;
            }
        }
        -1
    }
}

fn main() {}
