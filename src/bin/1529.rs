#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_flips(target: String) -> i32 {
        let mut flips = 0;
        let mut current_bit = '0';

        for bit in target.chars() {
            if bit != current_bit {
                flips += 1;
                current_bit = bit;
            }
        }

        flips
    }
}

fn main() {}
