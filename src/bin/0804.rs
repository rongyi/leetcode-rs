#![allow(dead_code)]
struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let dict: Vec<Vec<char>> = [
            ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
            "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-",
            "-.--", "--..",
        ]
        .into_iter()
        .map(|s| s.chars().collect())
        .collect();

        let words: Vec<Vec<char>> = words.into_iter().map(|s| s.chars().collect()).collect();

        let mut morse_code: HashSet<Vec<char>> = HashSet::new();
        for w in words.into_iter() {
            let mut cur: Vec<char> = Vec::new();
            for c in w.into_iter() {
                let idx = (c as u8 - 'a' as u8) as usize;
                let cur_morse = dict[idx].clone();
                cur.extend(cur_morse);
            }
            morse_code.insert(cur);
        }

        morse_code.len() as i32
    }
}

fn main() {}
