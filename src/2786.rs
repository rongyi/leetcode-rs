struct Solution;

use std::i64;
impl Solution {
    pub fn max_score(nums: Vec<i32>, x: i32) -> i64 {
        let mut max_even = -1e18 as i64;
        let mut max_odd = -1e18 as i64;
        if nums[0] % 2 == 0 {
            max_even = nums[0] as i64;
        } else {
            max_odd = nums[0] as i64;
        }
        let x = x as i64;
        for i in 1..nums.len() {
            let val = nums[i] as i64;
            if val % 2 == 0 {
                max_even = (max_even + val).max(max_odd + val - x);
            } else {
                max_odd = (max_odd + val).max(max_even + val - x);
            }
        }
        max_even.max(max_odd)
    }
}

fn main() {}
