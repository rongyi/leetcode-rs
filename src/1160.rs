#![allow(dead_code)]

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut dict: HashMap<char, i32> = HashMap::new();
        for c in chars.chars() {
            *dict.entry(c).or_insert(0) += 1;
        }
        let mut total_valid = 0;
        for w in words.into_iter() {
            let mut cur = HashMap::new();
            for c in w.chars() {
                *cur.entry(c).or_insert(0) += 1;
            }
            let mut valid = true;
            for (k, &v) in cur.iter() {
                if let Some(&dv) = dict.get(k) {
                    if v > dv {
                        valid = false;
                        break;
                    }
                } else {
                    valid = false;
                }
            }
            if valid {
                total_valid += w.len() as i32;
            }
        }
        total_valid
    }
}

fn main() {}
