#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut expected = nums[0] + 1;
        let mut ops = 0;
        for &num in nums.iter().skip(1) {
            if num < expected {
                ops += expected - num;
                expected = expected + 1;
            } else {
                expected = num + 1;
            }
        }

        ops
    }
}

fn main() {
    let input = vec![1, 1, 1];
    Solution::min_operations(input);
}
