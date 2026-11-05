use std::i32;

struct Solution;

impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut ret = nums[k as usize - 1] - nums[0];

        for i in 1..=nums.len() - k as usize {
            ret = ret.min(nums[i + k as usize - 1] - nums[i]);
        }

        ret
    }
}

fn main() {}
