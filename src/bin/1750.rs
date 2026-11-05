#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let s = s.as_bytes();
        let mut left = 0;
        let mut right = s.len() - 1;

        while left < right && s[left] == s[right] {
            let current_char = s[left];

            // Consume all consecutive occurrences of current_char from the left
            while left <= right && s[left] == current_char {
                left += 1;
            }

            // Consume all consecutive occurrences of current_char from the right
            while left <= right && s[right] == current_char {
                right -= 1;
            }
        }

        (right as i32 - left as i32 + 1) as i32
    }
}

fn main() {}
