#![allow(dead_code)]

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        let mut cnt: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            *cnt.entry(c).or_insert(0) += 1;
        }
        let mut odd_cnt = 0;
        for (_, v) in cnt.into_iter() {
            if v % 2 == 1 {
                odd_cnt += 1;
            }
        }
        odd_cnt <= k && k <= s.len() as i32
    }
}

fn main() {}
