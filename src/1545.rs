#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        if n == 1 || k == 1 {
            return '0';
        }

        let len = (1 << n) - 1; // Length of nth string: 2^n - 1
        let mid = (len + 1) / 2; // Middle position

        if k == mid {
            return '1'; // Middle bit is always 1 for n > 1
        } else if k < mid {
            // If k is in the first half, recursively find the bit in the (n-1)th string
            return Self::find_kth_bit(n - 1, k);
        } else {
            // If k is in the second half, find the corresponding bit in the (n-1)th string,
            // which is inverted and reversed
            let new_k = len - k + 1;
            let bit = Self::find_kth_bit(n - 1, new_k);
            // Invert the bit
            return if bit == '0' { '1' } else { '0' };
        }
    }
}

fn main() {}
