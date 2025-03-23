#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() as i32;
        let total_sum: i32 = nums.iter().sum();
        let mut result = vec![0; n as usize];
        let mut prefix_sum = 0;

        for i in 0..n as usize {
            let num = nums[i];

            // Each value contributes:
            // 1. The current value minus each value to its left
            // 2. Each value to its right minus the current value

            // Calculate left part: num * i - prefix_sum
            let left_part = num * (i as i32) - prefix_sum;

            // Calculate right part: (total_sum - prefix_sum - num) - num * (n - i - 1)
            let right_part = (total_sum - prefix_sum - num) - num * (n - (i as i32) - 1);

            result[i] = left_part + right_part;

            prefix_sum += num;
        }

        result
    }
}

fn main() {}
