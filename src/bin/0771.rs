
struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let valid: HashSet<char> = jewels.chars().collect();

        stones.chars().filter(|&c| valid.contains(&c)).count() as i32
    }
}

fn main() {}
