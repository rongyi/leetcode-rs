#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let nums: Vec<i32> = nums.into_iter().map(|i| i - 1).collect();
        let mut max_val = 0;
        let mut second_max = 0;

        for &val in nums.iter() {
            if val > max_val {
                second_max = max_val;
                max_val = val;
            } else if val > second_max {
                second_max = val;
            }
        }

        max_val * second_max
    }
}

fn main() {}
