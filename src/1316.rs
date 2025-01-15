#![allow(dead_code)]

use std::collections::HashSet;

// FIXME: almost TLE
impl Solution {
    pub fn distinct_echo_substrings(text: String) -> i32 {
        let mut uniq: HashSet<String> = HashSet::new();
        let s: Vec<char> = text.chars().collect();
        let sz = text.len();
        for len in 1..=sz / 2 {
            for i in 0..=sz - 2 * len {
                let mut same = true;
                for j in 0..len {
                    if s[i + j] != s[i + j + len] {
                        same = false;
                        break;
                    }
                }

                if same {
                    let cur: String = s[i..i + len].iter().copied().collect();
                    uniq.insert(cur);
                }
            }
        }

        uniq.len() as i32
    }
}

fn main() {}
