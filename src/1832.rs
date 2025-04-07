#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut map = [false; 26];
        for c in sentence.chars() {
            if c.is_ascii_lowercase() {
                map[c as usize - 'a' as usize] = true;
            }
        }
        map.iter().all(|&x| x)
    }
}

fn main() {}
