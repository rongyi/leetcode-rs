#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;

        // Try each possible value of x from 0 to n
        for x in 0..=n {
            let count = nums.iter().filter(|&&num| num >= x).count() as i32;

            if count == x {
                return x;
            }
        }

        -1
    }
}

fn main() {}
