#![allow(dead_code)]

struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut uniq: HashSet<String> = words.clone().into_iter().collect();

        for s in uniq.clone().iter() {
            let s: Vec<char> = s.chars().collect();
            for i in 1..s.len() {
                let cur: String = s[i..].to_vec().into_iter().collect();
                uniq.remove(&cur);
            }
        }

        let mut ret = 0;
        for s in uniq.iter() {
            ret += 1 + s.len() as i32;
        }

        ret
    }
}

fn main() {}
