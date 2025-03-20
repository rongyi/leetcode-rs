#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        let mut repeated_str = String::new();
        let mut k = 0;

        while repeated_str.len() <= sequence.len() {
            repeated_str.push_str(&word);
            if sequence.contains(&repeated_str) {
                k += 1;
            } else {
                break;
            }
        }

        k
    }
}

fn main() {}
