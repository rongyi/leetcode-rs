#![allow(dead_code)]

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let mut prev: HashMap<i32, i32> = HashMap::new();
        // mask 0 prev: -1, just for (j - i)
        prev.insert(0, -1);

        let mut max_len = 0;
        let mut mask = 0;
        for (idx, c) in s.chars().enumerate() {
            match c {
                'a' => mask ^= 1 << 0,
                'o' => mask ^= 1 << 1,
                'e' => mask ^= 1 << 2,
                'i' => mask ^= 1 << 3,
                'u' => mask ^= 1 << 4,
                _ => (),
            }
            if let Some(prev_idx) = prev.get(&mask) {
                max_len = max_len.max(idx as i32 - prev_idx);
            } else {
                prev.insert(mask, idx as i32);
            }
        }

        max_len
    }
}

fn main() {}
