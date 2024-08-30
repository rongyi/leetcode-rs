struct Solution;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }
        if s == goal {
            let mut seen = HashSet::new();
            return s.chars().any(|c| !seen.insert(c));
        }

        let diff: Vec<(char, char)> = s
            .chars()
            .zip(goal.chars())
            .filter(|&(a, b)| a != b)
            .collect();

        diff.len() == 2 && diff[0].0 == diff[1].1 && diff[0].1 == diff[1].0
    }
}

fn main() {
    Solution::buddy_strings("ab".to_string(), "ba".to_string());
}
