#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn thousand_separator(n: i32) -> String {
        let s = n.to_string();
        let len = s.len();

        if len <= 3 {
            return s;
        }

        let mut result = String::new();
        for (i, c) in s.chars().enumerate() {
            result.push(c);
            if (len - i - 1) % 3 == 0 && i < len - 1 {
                result.push('.');
            }
        }

        result
    }
}

fn main() {}
