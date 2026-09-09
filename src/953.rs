struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut sorted: HashMap<char, usize> = HashMap::new();
        for (i, c) in order.chars().enumerate() {
            sorted.insert(c, i);
        }

        let mut normalized: Vec<Vec<usize>> = Vec::with_capacity(words.len());
        for w in words.iter() {
            let cur: Vec<usize> = w.chars().map(|c| sorted[&c]).collect();
            normalized.push(cur);
        }

        let mut sorted_normalize: Vec<Vec<usize>> = normalized.clone();
        sorted_normalize.sort_unstable();

        sorted_normalize == normalized
    }
}

fn main() {}
