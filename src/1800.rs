#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut max_sum = nums[0];
        let mut current_sum = nums[0];

        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                current_sum += nums[i];
            } else {
                current_sum = nums[i];
            }
            max_sum = max_sum.max(current_sum);
        }

        max_sum
    }
}

fn main() {}
