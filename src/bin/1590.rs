#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let p = p as i64;
        let total_rem = nums.iter().map(|&x| x as i64).sum::<i64>() % p;

        if total_rem == 0 {
            return 0;
        }

        let n = nums.len();
        let mut min_len = n as i32;
        let mut prefix_sum = 0;
        let mut remainder_map = std::collections::HashMap::new();

        // Initialize with 0 remainder at position -1
        remainder_map.insert(0, -1);

        for (i, &num) in nums.iter().enumerate() {
            prefix_sum = (prefix_sum + num as i64) % p;

            // Find if there's a prefix with remainder (prefix_sum - total_rem) % p
            let target = (prefix_sum - total_rem + p) % p;

            if let Some(&j) = remainder_map.get(&target) {
                min_len = min_len.min((i as i32) - j);
            }

            // Store current prefix sum remainder with its position
            remainder_map.insert(prefix_sum, i as i32);
        }

        if min_len == n as i32 {
            return -1;
        }

        min_len
    }
}

fn main() {}
