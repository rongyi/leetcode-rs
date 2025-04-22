#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let mut words: Vec<_> = s.split(' ').collect();

        words.sort_by_key(|word| word.chars().last().unwrap().to_digit(10).unwrap());

        let mut result = String::new();
        for word in words {
            result.push_str(&word[..word.len() - 1]);
            result.push(' ');
        }
        result.pop();

        result
    }
}

fn main() {}
