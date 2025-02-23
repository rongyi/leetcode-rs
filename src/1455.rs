#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        let words: Vec<_> = sentence.split(' ').collect();

        for (i, w) in words.iter().enumerate() {
            if w.starts_with(&search_word) {
                return (i + 1) as i32;
            }
        }

        -1
    }
}

fn main() {}
