#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        let mut s: Vec<char> = palindrome.chars().collect();
        let sz = s.len();
        let mut has_change = false;
        for i in 0..sz / 2 {
            if s[i] > 'a' {
                s[i] = 'a';
                has_change = true;
                break;
            }
        }

        if has_change {
            s.into_iter().collect()
        } else {
            if sz <= 1 {
                "".to_string()
            } else {
                // it must be 'aaaaa' this form, so just change the last one
                s[sz - 1] = 'b';
                s.into_iter().collect()
            }
        }
    }
}

fn main() {}
