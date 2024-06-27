#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        Self::count(&nums, right) - Self::count(&nums, left - 1)
    }
    fn count(nums: &Vec<i32>, bound: i32) -> i32 {
        let mut ret = 0;
        let mut cur_acc = 0;
        for &num in nums.iter() {
            if num <= bound {
                cur_acc += 1;
            } else {
                cur_acc = 0;
            }
            ret += cur_acc;
        }

        ret
    }
}

fn main() {
    let input = vec![1, 1, 1];
}
