#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn is_ideal_permutation(nums: Vec<i32>) -> bool {
        let sz = nums.len();
        let mut cur_min = i32::MAX;
        for i in (2..sz).rev() {
            cur_min = cur_min.min(nums[i]);
            if nums[i - 2] > cur_min {
                return false;
            }
        }

        true
    }
}

fn main() {}
