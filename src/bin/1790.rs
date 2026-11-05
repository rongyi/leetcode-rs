#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        if s1 == s2 {
            return true;
        }

        if s1.len() != s2.len() {
            return false;
        }

        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();

        let mut diff_indices = Vec::new();

        for i in 0..s1_chars.len() {
            if s1_chars[i] != s2_chars[i] {
                diff_indices.push(i);
            }

            if diff_indices.len() > 2 {
                return false;
            }
        }

        match diff_indices.len() {
            0 => true,
            2 => {
                let i = diff_indices[0];
                let j = diff_indices[1];
                s1_chars[i] == s2_chars[j] && s1_chars[j] == s2_chars[i]
            }
            _ => false,
        }
    }
}
fn main() {}
