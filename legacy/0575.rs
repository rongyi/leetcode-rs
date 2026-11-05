struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let uniq: HashSet<i32> = candy_type.iter().cloned().collect();

        uniq.len().min(candy_type.len() / 2) as i32
    }
}

fn main() {}
