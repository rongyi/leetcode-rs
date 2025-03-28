#![allow(dead_code)]

struct Solution;

impl Solution {
    // category: binary search
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        let mut left = 1;
        let mut right = *nums.iter().max().unwrap();

        while left < right {
            let mid = left + (right - left) / 2;

            let operations = nums.iter().map(|&num| (num - 1) / mid).sum::<i32>();

            if operations <= max_operations {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left
    }
}

fn main() {}
