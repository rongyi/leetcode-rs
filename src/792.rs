#![allow(dead_code)]
struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let sz = s.len();
        let mut ret = 0;
        let mut valid: HashSet<Vec<char>> = HashSet::new();
        let mut invalid: HashSet<Vec<char>> = HashSet::new();

        for w in words.iter() {
            let w: Vec<char> = w.chars().collect();
            if valid.contains(&w) {
                ret += 1;
                continue;
            }
            if invalid.contains(&w) {
                continue;
            }
            let cur_sz = w.len();
            let mut i = 0;
            let mut j = 0;
            while i < sz && j < cur_sz {
                if s[i] == w[j] {
                    j += 1;
                }
                i += 1;
            }
            // yes, a valid subsequence
            if j == cur_sz {
                ret += 1;
                valid.insert(w);
            } else {
                invalid.insert(w);
            }
        }

        ret
    }
}

fn main() {}
