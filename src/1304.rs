#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut nums: Vec<i32> = (1..=n - 1).collect();
        nums.push(-(n * (n - 1) / 2));

        nums
    }
}

fn main() {}
