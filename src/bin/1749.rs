#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut max_sum = 0;
        let mut min_sum = 0;
        let mut curr_max = 0;
        let mut curr_min = 0;

        for &num in &nums {
            curr_max = num.max(curr_max + num);
            curr_min = num.min(curr_min + num);
            max_sum = max_sum.max(curr_max);
            min_sum = min_sum.min(curr_min);
        }

        max_sum.max(-min_sum)
    }
}

fn main() {}
