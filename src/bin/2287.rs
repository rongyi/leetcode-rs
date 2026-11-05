
struct Solution;

use std::{collections::HashMap, i32};
impl Solution {
    pub fn rearrange_characters(s: String, target: String) -> i32 {
        let mut material: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            *material.entry(c).or_default() += 1;
        }
        let mut group = i32::MAX;
        let mut exist: HashMap<char, i32> = HashMap::new();
        for c in target.chars() {
            *exist.entry(c).or_default() += 1;
        }
        for (k, &v) in exist.iter() {
            if let Some(&total_v) = material.get(k) {
                group = group.min(total_v / v);
            } else {
                return 0;
            }
        }

        group
    }
}

fn main() {}
