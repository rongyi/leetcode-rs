#![allow(dead_code)]

struct Solution;

impl Solution {
    // leetcode 1513
    pub fn num_sub(s: String) -> i32 {
        let mut count = 0;
        let mut consecutive = 0;

        for c in s.chars() {
            if c == '1' {
                consecutive += 1;
                count += consecutive;
                count %= 1_000_000_007;
            } else {
                consecutive = 0;
            }
        }

        count
    }
}

fn main() {}
