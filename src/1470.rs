#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let sz = nums.len();
        let mut ret = Vec::with_capacity(sz);
        for i in 0..sz / 2 {
            ret.push(nums[i]);
            ret.push(nums[i + sz / 2]);
        }

        ret
    }
}

fn main() {}
