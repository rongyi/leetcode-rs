#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        let modulo = 1_000_000_007;
        let mut result: i64 = 0;

        for i in 1..=n {
            // Calculate the length of the binary representation of current number
            let len = 32 - i.leading_zeros();

            // Update result by shifting left by len bits and adding the current number
            result = ((result << len) | i as i64) % modulo;
        }

        result as i32
    }
}

fn main() {}
