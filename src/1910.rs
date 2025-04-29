#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let mut result = s;
        while result.contains(&part) {
            if let Some(i) = result.find(&part) {
                result = result[..i].to_string() + &result[i + part.len()..];
            }
        }
        result
    }
}

fn main() {}
