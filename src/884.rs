struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut set1: HashMap<_, i32> = HashMap::new();
        let mut set2: HashMap<_, i32> = HashMap::new();

        for w in s1.split(' ') {
            *set1.entry(w).or_default() += 1;
        }
        for w in s2.split(' ') {
            *set2.entry(w).or_default() += 1;
        }
        let mut out = vec![];

        for (k, &v) in set1.iter() {
            if v == 1 && !set2.contains_key(k) {
                out.push(k.to_string());
            }
        }

        for (k, &v) in set2.iter() {
            if v == 1 && !set1.contains_key(k) {
                out.push(k.to_string());
            }
        }

        out
    }
}

fn main() {}
