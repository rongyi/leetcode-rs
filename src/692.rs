struct Solution;

use std::collections::{BTreeMap, BTreeSet, HashMap};

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut cnt: HashMap<String, i32> = HashMap::new();
        for w in words.into_iter() {
            *cnt.entry(w).or_insert(0) += 1;
        }
        let mut group_cnt: BTreeMap<i32, BTreeSet<String>> = BTreeMap::new();
        for (k, &v) in cnt.iter() {
            group_cnt
                .entry(v)
                .or_insert(BTreeSet::new())
                .insert(k.clone());
        }
        let mut ret: Vec<String> = Vec::new();
        for (_, v) in group_cnt.iter().rev() {
            for w in v.iter() {
                if ret.len() >= k as usize {
                    break;
                }
                ret.push(w.clone());
            }
        }

        ret
    }
}

fn main() {}
