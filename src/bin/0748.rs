#![allow(dead_code)]

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let license_plate = license_plate.to_lowercase();
        let mut lp: HashMap<char, i32> = HashMap::new();
        let mut shortest_word = String::new();
        let mut shortest_len = usize::MAX;
        for c in license_plate.chars() {
            if c.is_alphabetic() {
                *lp.entry(c).or_insert(0) += 1;
            }
        }

        for (i, w) in words.iter().enumerate() {
            let mut cur_cnt: HashMap<char, i32> = HashMap::new();
            for c in w.chars() {
                if c.is_alphabetic() {
                    *cur_cnt.entry(c).or_insert(0) += 1;
                }
            }
            let mut ok = true;
            for (c, &cnt) in lp.iter() {
                if *cur_cnt.get(c).unwrap_or(&0) < cnt {
                    ok = false;
                    break;
                }
            }

            if ok && w.len() < shortest_len {
                shortest_len = w.len();
                shortest_word = w.clone();
            }
        }

        shortest_word
    }
}

fn main() {}
