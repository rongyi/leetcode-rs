#![allow(dead_code)]

struct Master;
impl Master {
    fn guess(word: String) -> i32 {
        1
    }
}
struct Solution;

/**
 * // This is the Master's API interface.
 * // You should not implement it, or speculate about its implementation
 * struct Master;
 * impl Master {
 *     fn guess(word:String)->int;
 * };
 */
use rand::{self, Rng};
impl Solution {
    pub fn find_secret_word(words: Vec<String>, master: &Master) {
        let mut rng = rand::thread_rng();
        let mut candidates = words.clone();
        while !candidates.is_empty() {
            let word = &candidates[rng.gen_range(0..candidates.len())];
            let match_cnt = master.guess(word.clone());
            if match_cnt == 6 {
                return;
            }
            let tmp: Vec<String> = candidates
                .iter()
                .filter(|&w| Self::find_matches(w, word) == match_cnt)
                .cloned()
                .collect();
            candidates = tmp;
        }
    }

    fn find_matches(a: &str, b: &str) -> i32 {
        a.chars()
            .zip(b.chars())
            .filter(|&(c1, c2)| c1 == c2)
            .count() as i32
    }
}

fn main() {}
