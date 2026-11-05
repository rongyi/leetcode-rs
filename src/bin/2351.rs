struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut set: HashSet<char> = HashSet::new();
        for c in s.chars() {
            // the second time meet
            if set.contains(&c) {
                return c;
            }
            set.insert(c);
        }
        unreachable!()
    }
}

fn main() {}
