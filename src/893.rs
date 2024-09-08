
struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn num_special_equiv_groups(words: Vec<String>) -> i32 {
        let mut same: HashSet<Vec<char>> = HashSet::new();
        for w in words.into_iter() {
            let mut even: Vec<char> = w.chars().step_by(2).collect();
            let mut odd: Vec<char> = w.chars().skip(1).step_by(2).collect();
            even.sort_unstable();
            odd.sort_unstable();
            odd.append(&mut even);
            same.insert(odd);
        }

        same.len() as i32
    }
}

fn main() {}
