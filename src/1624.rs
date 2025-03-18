#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut first_occurrence = [-1; 26]; // Use an array for lowercase letters
        let mut max_length = -1;

        for (i, &c) in chars.iter().enumerate() {
            let index = (c as usize) - ('a' as usize);

            if first_occurrence[index] == -1 {
                // First occurrence of this character
                first_occurrence[index] = i as i32;
            } else {
                // Calculate length between equal characters
                let length = (i as i32) - first_occurrence[index] - 1;
                max_length = max_length.max(length);
            }
        }

        max_length
    }
}

fn main() {}
