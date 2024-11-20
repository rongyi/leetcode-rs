
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut cnt = HashMap::new();
        for dm in dominoes.into_iter() {
            *cnt.entry((dm[0].min(dm[1]), dm[0].max(dm[1]))).or_insert(0) += 1;
        }

        cnt.into_iter().fold(0, |mut acc, ((_x, _y), cnt)| {
            acc += (cnt - 1) * cnt / 2;
            acc
        })
    }
}

fn main() {}
