#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn make_good(s: String) -> String {
        let mut stack: Vec<char> = Vec::new();

        for c in s.chars() {
            if !stack.is_empty()
                && stack.last().unwrap().to_ascii_lowercase() == c.to_ascii_lowercase()
                && stack.last().unwrap() != &c
            {
                // Found a pair of same letter with different cases
                stack.pop();
            } else {
                stack.push(c);
            }
        }

        stack.iter().collect()
    }
}

fn main() {}
