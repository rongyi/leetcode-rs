#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        let m = 1_000_000_007;
        let mut total = 0;
        let mut count = 0;
        let mut prev = ' ';

        for c in s.chars() {
            if c == prev {
                count += 1;
            } else {
                count = 1;
                prev = c;
            }

            total = (total + count) % m;
        }

        total
    }
}

fn main() {}
