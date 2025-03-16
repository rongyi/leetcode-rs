#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        let mut result = 0;
        let mut n = n;

        // This algorithm uses the Gray code transformation property.
        // Gray code is a binary numeral system where two successive values differ in only one bit.
        // For a number n, the minimum operations needed to reduce it to 0 is its Gray code.
        // The formula to convert a binary number to its Gray code is: g(n) = n ^ (n >> 1)
        // However, here we're implementing it iteratively using the XOR operation.

        while n > 0 {
            result ^= n;
            n >>= 1;
        }

        result
    }
}

fn main() {}
