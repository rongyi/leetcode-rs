#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut rotations = 0;

        for i in 0..n - 1 {
            if nums[i] > nums[i + 1] {
                rotations += 1;
            }
        }

        // Check if the last element is greater than the first
        if nums[n - 1] > nums[0] {
            rotations += 1;
        }

        // If array is sorted in non-decreasing order, or can be after one rotation
        rotations <= 1
    }
}

fn main() {}
