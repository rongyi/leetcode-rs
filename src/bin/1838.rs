#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {
        // Sort the array
        nums.sort_unstable();

        let mut left = 0;
        let mut max_freq = 0;
        let mut operations_sum = 0;

        // Sliding window approach
        for right in 0..nums.len() {
            // If this is not the first element, add operations needed
            // to make the previous element equal to the current one
            if right > 0 {
                operations_sum +=
                    (nums[right] - nums[right - 1]) as i64 * (right as i64 - left as i64);
            }

            // Shrink window if we exceed k operations
            while operations_sum > k as i64 {
                operations_sum -= (nums[right] - nums[left]) as i64;
                left += 1;
            }

            // Update max frequency based on window size
            max_freq = max_freq.max(right - left + 1);
        }

        max_freq as i32
    }
}

fn main() {}
