#![allow(dead_code)]

struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        let mut words: Vec<Vec<char>> = sentence.split(" ").map(|s| s.chars().collect()).collect();
        let vowels: HashSet<char> = "aoeiuAOEIU".chars().collect();
        let ma: Vec<char> = "ma".chars().collect();

        let mut latin_words: Vec<String> = Vec::new();

        for (i, w) in words.iter_mut().enumerate() {
            let mut need_skip = false;
            if !vowels.contains(&w[0]) {
                w.push(w[0]);
                need_skip = true;
            }

            w.extend(&ma);

            for _ in 0..=i {
                w.push('a');
            }

            let cur = w.iter().skip(if need_skip { 1 } else { 0 }).collect();
            latin_words.push(cur);
        }

        latin_words.join(" ")
    }
}

fn main() {}
