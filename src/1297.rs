#![allow(dead_code)]

struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn max_freq(s: String, max_letters: i32, min_size: i32, max_size: i32) -> i32 {
        let mut cnt: HashMap<String, i32> = HashMap::new();
        let mut ret = 0;

        let s: Vec<char> = s.chars().collect();
        let min_size = min_size as usize;
        let max_letter = max_letters as usize;

        for i in 0..=s.len() - min_size {
            let uniq: HashSet<char> = s[i..i + min_size].iter().cloned().collect();
            if uniq.len() > max_letter {
                continue;
            }
            // now a valid substring
            let key: String = s[i..i + min_size].iter().cloned().collect();
            *cnt.entry(key.clone()).or_insert(0) += 1;

            ret = ret.max(*cnt.get(&key).unwrap());
        }

        ret
    }
}

fn main() {}
