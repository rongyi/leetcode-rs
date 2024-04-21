#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut mul = 1;
        let mut i = 0;
        let mut ret = 0;
        for j in 0..nums.len() {
            mul *= nums[j];
            while i <= j && mul >= k {
                mul /= nums[i];
                i += 1;
            }
            ret += j - i + 1;
        }
        ret as i32
    }
}

fn main() {}
