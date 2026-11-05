#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();

        let n = nums.len();
        let mut max_sum = 0;

        for i in 0..n / 2 {
            max_sum = max_sum.max(nums[i] + nums[n - 1 - i]);
        }

        max_sum
    }
}

fn main() {}
