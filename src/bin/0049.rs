struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut group: HashMap<String, Vec<String>> = HashMap::new();

        for s in strs.into_iter() {
            let mut s1: Vec<char> = s.chars().collect();
            s1.sort_unstable();
            let k: String = s1.into_iter().collect();
            group.entry(k).or_default().push(s);
        }

        group.into_values().collect()
    }
}

fn main() {}
