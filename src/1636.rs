#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;

        // Count frequencies
        let mut freq_map: HashMap<i32, i32> = HashMap::new();
        for &num in &nums {
            *freq_map.entry(num).or_insert(0) += 1;
        }

        // Sort the nums
        let mut result = nums.clone();
        result.sort_by(|a, b| {
            let freq_a = freq_map.get(a).unwrap();
            let freq_b = freq_map.get(b).unwrap();

            // If frequencies are different, sort by frequency (ascending)
            if freq_a != freq_b {
                return freq_a.cmp(freq_b);
            }

            // If frequencies are the same, sort by value (descending)
            return b.cmp(a);
        });

        result
    }
}

fn main() {}
