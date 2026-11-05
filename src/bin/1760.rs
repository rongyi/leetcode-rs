#![allow(dead_code)]

struct Solution;

impl Solution {
    // category: binary search
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        let mut left = 1;
        let mut right = *nums.iter().max().unwrap();

        while left < right {
            let mid = left + (right - left) / 2;
            //
            // To find the number of operations needed for each number:
            // (num - 1) / mid gives the number of operations needed to reduce `num` below or equal to `mid`
            // For example, if num=10 and mid=3, we need to split 10 into [3,3,3, 1] which takes 3 operations
            // This can be calculated as (10-1)/3 = 3 operations            ^ ^ ^ <- three cut
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
