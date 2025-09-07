
struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn shortest_sequence(rolls: Vec<i32>, k: i32) -> i32 {
        let mut uniq: HashSet<i32> = HashSet::new();
        let mut full_group = 0;

        for &r in rolls.iter() {
            uniq.insert(r);
            if uniq.len() == k as usize {
                full_group += 1;
                uniq.clear();
            }
        }

        full_group + 1
    }
}

fn main() {}
