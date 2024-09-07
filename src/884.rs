
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let w1: Vec<String> = s1.split(' ').map(|s| s.to_string()).collect();
        let w2: Vec<String> = s2.split(' ').map(|s| s.to_string()).collect();
        let mut cnt: HashMap<String, i32> = HashMap::new();
        for w in w1.into_iter().chain(w2.into_iter()) {
            *cnt.entry(w).or_insert(0) += 1;
        }

        cnt.into_iter()
            .filter(|&(_, v)| v == 1)
            .map(|(k, _v)| k)
            .collect()
    }
}

fn main() {}
