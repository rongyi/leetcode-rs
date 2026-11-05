#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn reformat(s: String) -> String {
        let mut digits: Vec<char> = vec![];
        let mut letters: Vec<char> = vec![];
        for c in s.chars() {
            if c.is_ascii_digit() {
                digits.push(c);
            } else if c.is_ascii_lowercase() {
                letters.push(c);
            }
        }

        if (digits.len() as i32 - letters.len() as i32).abs() > 1 {
            return "".to_string();
        }

        let mut result = String::new();
        let (mut longer, mut shorter) = if letters.len() > digits.len() {
            (letters, digits)
        } else {
            (digits, letters)
        };

        // Alternate between longer and shorter vectors
        while !longer.is_empty() || !shorter.is_empty() {
            if let Some(c) = longer.pop() {
                result.push(c);
            }
            if let Some(c) = shorter.pop() {
                result.push(c);
            }
        }

        result
    }
}

fn main() {}
