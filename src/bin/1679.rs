#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;

        let mut count_map = HashMap::new();
        let mut operations = 0;

        for &num in &nums {
            let complement = k - num;

            if let Some(&count) = count_map.get(&complement) {
                if count > 0 {
                    operations += 1;
                    count_map.insert(complement, count - 1);
                } else {
                    *count_map.entry(num).or_insert(0) += 1;
                }
            } else {
                *count_map.entry(num).or_insert(0) += 1;
            }
        }

        operations
    }
}

fn main() {}
