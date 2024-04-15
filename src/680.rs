#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            if s[i] != s[j] {
                return Self::is_palindrom(&s[i + 1..j + 1]) || Self::is_palindrom(&s[i..j]);
            }
            i += 1;
            j -= 1;
        }
        true
    }
    fn is_palindrom(s: &[char]) -> bool {
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            if s[i] != s[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}

fn main() {}
