#![allow(dead_code)]

struct Solution;

use std::collections::BTreeMap;
impl Solution {
    pub fn is_possible_divide(nums: Vec<i32>, k: i32) -> bool {
        let mut cnt: BTreeMap<i32, i32> = BTreeMap::new();

        for &num in nums.iter() {
            *cnt.entry(num).or_insert(0) += 1;
        }
        // lengh k is given which is way better!
        while !cnt.is_empty() {
            let start = *cnt.keys().next().unwrap();
            for i in 0..k {
                let cur = start + i;
                if !cnt.contains_key(&cur) {
                    return false;
                } else {
                    if *cnt.get(&cur).unwrap() == 1 {
                        cnt.remove(&cur);
                    } else {
                        *cnt.get_mut(&cur).unwrap() -= 1;
                    }
                }
            }
        }

        true
    }
}

fn main() {}
