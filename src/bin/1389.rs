#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let sz = nums.len();
        let mut ret: Vec<i32> = Vec::with_capacity(sz);

        for (&val, &idx) in nums.iter().zip(index.iter()) {
            ret.insert(idx as usize, val);
        }

        ret
    }
}

fn main() {}
