#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_product_difference(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let sz = nums.len();
        nums[sz - 1] * nums[sz - 2] - nums[0] * nums[1]
    }
}

fn main() {}
