#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn beauty_sum(s: String) -> i32 {
        let mut total = 0;
        let n = s.len();
        let s_bytes = s.as_bytes();

        for i in 0..n {
            let mut freq = [0; 26];
            for j in i..n {
                freq[(s_bytes[j] - b'a') as usize] += 1;

                let mut min_freq = n;
                let mut max_freq = 0;

                for &f in freq.iter().filter(|&&f| f > 0) {
                    min_freq = min_freq.min(f);
                    max_freq = max_freq.max(f);
                }

                total += max_freq - min_freq;
            }
        }

        total as _
    }
}

fn main() {}
