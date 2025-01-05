#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .map(|x| x.to_string().len())
            .filter(|x| x % 2 == 0)
            .count() as i32
    }
}

fn main() {}
