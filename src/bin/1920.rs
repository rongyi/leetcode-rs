#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let sz = nums.len();
        let mut ret = vec![0; sz];

        for i in 0..sz {
            ret[i] = nums[nums[i] as usize];
        }

        ret
    }
}
fn main() {}
