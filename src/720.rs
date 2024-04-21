#![allow(dead_code)]

struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn longest_word(mut words: Vec<String>) -> String {
        words.sort_unstable();
        let mut cache = HashSet::new();
        let mut ret = String::new();
        for w in words.into_iter() {
            let mut prev = w.clone();
            prev.pop();
            if w.len() == 1 || cache.contains(&prev) {
                if w.len() > ret.len() {
                    ret = w.clone();
                }
                cache.insert(w);
            }
        }

        ret
    }
}

fn main() {}
