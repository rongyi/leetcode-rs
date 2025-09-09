
struct Solution;

use std::collections::BTreeMap;
impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut group: BTreeMap<i32, i32> = BTreeMap::new();
        for p in items1.iter().chain(items2.iter()) {
            *group.entry(p[0]).or_default() += p[1];
        }

        group.into_iter().fold(vec![], |mut acc, cur| {
            acc.push(vec![cur.0, cur.1]);
            acc
        })
    }
}

fn main() {}
