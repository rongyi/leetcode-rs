#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut acc = 0;
        let mut max = 0;

        for &x in &gain {
            acc += x;
            max = max.max(acc);
        }

        max
    }
}

fn main() {}
