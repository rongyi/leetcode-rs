struct Solution;
use std::collections::BTreeMap;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let s: Vec<char> = s.chars().collect();
        let mut cnt: BTreeMap<char, i32> = BTreeMap::new();
        let mut group: BTreeMap<i32, Vec<char>> = BTreeMap::new();
        for c in s.into_iter() {
            *cnt.entry(c).or_insert(0) += 1;
        }
        for (k, v) in cnt.into_iter() {
            group.entry(v).or_insert(Vec::new()).push(k);
        }
        let mut ret = String::new();
        for (k, v) in group.iter().rev() {
            let cur: String = v
                .iter()
                .map(|c| c.to_string().repeat(*k as usize))
                .collect();
            ret.push_str(&cur);
        }

        ret
    }
}

fn main() {}
