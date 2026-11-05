#![allow(dead_code)]
struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn num_splits(s: String) -> i32 {
        let s_bytes = s.as_bytes();

        // Count distinct characters to the right of each position
        // including itself
        let mut right_distinct = vec![0; s.len()];
        let mut right_chars = HashSet::new();

        for i in (0..s.len()).rev() {
            right_chars.insert(s_bytes[i]);
            right_distinct[i] = right_chars.len();
        }

        // Count good splits
        let mut left_chars = HashSet::new();
        let mut good_splits = 0;

        for i in 0..(s.len() - 1) {
            left_chars.insert(s_bytes[i]);
            if left_chars.len() == right_distinct[i + 1] {
                good_splits += 1;
            }
        }

        good_splits
    }
}

fn main() {}
