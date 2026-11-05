
struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let mut meet_again: HashSet<char> = HashSet::new();
        let mut chunk = 1;
        // let mut cur_start = 1;
        for (i, c) in s.chars().enumerate() {
            if meet_again.contains(&c) {
                meet_again.clear();
                chunk += 1;
                // cur_start = i;
            }
            meet_again.insert(c);
        }
        chunk
    }
}

fn main() {}
