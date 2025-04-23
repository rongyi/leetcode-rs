#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        if s.len() < 3 {
            return 0;
        }

        let chars: Vec<char> = s.chars().collect();
        let mut count = 0;

        for i in 0..=s.len() - 3 {
            if chars[i] != chars[i + 1] && chars[i] != chars[i + 2] && chars[i + 1] != chars[i + 2]
            {
                count += 1;
            }
        }

        count
    }
}

fn main() {}
