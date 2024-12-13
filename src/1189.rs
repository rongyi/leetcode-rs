#![allow(dead_code)]


struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let s = "balloon";
        let mut base: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            *base.entry(c).or_insert(0) += 1;
        }
        let mut candidate: HashMap<char, i32> = HashMap::new();
        for c in text.chars() {
            *candidate.entry(c).or_insert(0) += 1;
        }
        let mut total_group = i32::MAX;

        for c in s.chars() {
            let cur_candi = *candidate.get(&c).unwrap_or(&0);
            let need = *base.get(&c).unwrap();
            total_group = total_group.min(cur_candi / need);
        }

        total_group
    }
}

fn main() {}
