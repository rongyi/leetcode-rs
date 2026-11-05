struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        // Pre-compute all powers of 2 and their digit frequencies
        // Up to 10^9 (since n <= 10^9)
        let mut power_of_2_freqs = Vec::new();
        let mut power = 1;

        // 2^30 = 1,073,741,824 > 10^9, so we stop at 2^29
        while power <= 1_000_000_000 {
            power_of_2_freqs.push(Self::get_digit_freq(power));
            power <<= 1;
        }

        let n_freq = Self::get_digit_freq(n);
        power_of_2_freqs.contains(&n_freq)
    }

    fn get_digit_freq(mut num: i32) -> [i32; 10] {
        let mut freq = [0; 10];
        while num > 0 {
            let digit = (num % 10) as usize;
            freq[digit] += 1;
            num /= 10;
        }
        freq
    }
}

fn main() {}
