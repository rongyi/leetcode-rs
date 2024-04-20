#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn to_lower_case(s: String) -> String {
        let mut s: Vec<char> = s.chars().collect();
        for c in s.iter_mut() {
            if c.is_uppercase() {
                *c = ('a' as u8 + (*c as u8 - 'A' as u8)) as char;
            }
        }
        s.into_iter().collect()
    }
}

fn main() {}
