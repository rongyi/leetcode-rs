
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let word_set: Vec<HashMap<char, usize>> = words
            .into_iter()
            .map(|s| {
                s.chars().fold(HashMap::new(), |mut acc, cur| {
                    *acc.entry(cur).or_insert(0) += 1;
                    acc
                })
            })
            .collect();
        let mut ret = Vec::new();
        for c in 'a'..='z' {
            let vals: Vec<usize> = word_set
                .iter()
                .map(|set| set.get(&c).unwrap_or(&0))
                .copied()
                .collect();
            let &val = vals.iter().min().unwrap();
            for _ in 0..val {
                ret.push(c.to_string());
            }
        }

        ret
    }
}

fn main() {}
