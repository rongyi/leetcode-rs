#![allow(dead_code)]
struct Solution;

impl Solution {
    // nonsense, dont want to learn this trick
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut prev = 0;

        for &num in &target {
            if num > prev {
                result += num - prev;
            }
            prev = num;
        }

        result
    }
}

fn main() {}
