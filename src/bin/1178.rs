#![allow(dead_code)]

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        let letter_mask = |idx: u8| -> usize { 1 << idx };
        // let mut letter_mask = vec![0; 26];
        let mut words_mask: HashMap<usize, i32> = HashMap::new();
        for w in words.iter() {
            let mut cur_mask = 0;
            for c in w.chars() {
                cur_mask |= letter_mask(c as u8 - 'a' as u8);
            }
            *words_mask.entry(cur_mask).or_insert(0) += 1;
        }
        let mut ret = vec![0; puzzles.len()];

        for (i, p) in puzzles.iter().enumerate() {
            let mut cur_mask = 0;
            let p: Vec<char> = p.chars().collect();
            for &c in p.iter() {
                cur_mask |= letter_mask(c as u8 - 'a' as u8);
            }

            // just keep find subset against all words
            // words letter is a subset of puzzle letter
            let mut iter_mask = cur_mask;
            while iter_mask > 0 {
                // first letter must same
                if (iter_mask & letter_mask(p[0] as u8 - 'a' as u8)) != 0 {
                    ret[i] += *words_mask.get(&iter_mask).unwrap_or(&0);
                }

                iter_mask = (iter_mask - 1) & cur_mask;
            }
        }

        ret
    }
}

fn main() {}
