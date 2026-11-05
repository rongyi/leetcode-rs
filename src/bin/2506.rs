
struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn similar_pairs(words: Vec<String>) -> i32 {
        let words: Vec<HashSet<char>> = words
            .iter()
            .map(|s| s.chars().collect::<HashSet<char>>())
            .collect();
        let mut ret = 0;
        for i in 0..words.len() {
            for j in i + 1..words.len() {
                if words[i].eq(&words[j]) {
                    ret += 1;
                }
            }
        }
        ret
    }
}

fn main() {}
