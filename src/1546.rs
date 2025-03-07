#![allow(dead_code)]

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_non_overlapping(nums: Vec<i32>, target: i32) -> i32 {
        let mut result = 0;
        let mut prefix_sum = 0;
        let mut sum_map: HashMap<i32, i32> = HashMap::new();

        // Initialize with sum 0 seen at position -1
        sum_map.insert(0, -1);

        let mut last_end = -1;

        for (i, &num) in nums.iter().enumerate() {
            prefix_sum += num;

            // Look for a previous subarray that would make current sum - previous sum = target
            if let Some(&j) = sum_map.get(&(prefix_sum - target)) {
                // Check if previous subarray endpoint is after our last chosen subarray
                if j >= last_end {
                    result += 1;
                    last_end = i as i32;
                }
            }

            // Update the latest position for the current sum
            sum_map.insert(prefix_sum, i as i32);
        }

        result
    }
}

fn main() {}
