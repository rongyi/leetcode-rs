#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = String::new();
        let mut i = 0;
        let mut j = 0;
        let word1: Vec<char> = word1.chars().collect();
        let word2: Vec<char> = word2.chars().collect();
        while i < word1.len() && j < word2.len() {
            result.push(word1[i]);
            result.push(word2[j]);
            i += 1;
            j += 1;
        }
        if i < word1.len() {
            result.push_str(&word1[i..].iter().clone().collect::<String>());
        }
        if j < word2.len() {
            result.push_str(&word2[j..].iter().clone().collect::<String>());
        }

        result
    }
}

fn main() {}
