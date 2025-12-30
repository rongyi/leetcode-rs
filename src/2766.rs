struct Solution;

use std::collections::BTreeSet;
impl Solution {
    pub fn relocate_marbles(nums: Vec<i32>, move_from: Vec<i32>, move_to: Vec<i32>) -> Vec<i32> {
        let mut positions: BTreeSet<i32> = BTreeSet::new();

        for &num in nums.iter() {
            positions.insert(num);
        }
        for i in 0..move_from.len() {
            let (from, to) = (move_from[i], move_to[i]);
            // delete all marlble in from position and update postion to to
            positions.remove(&from);
            positions.insert(to);
        }

        positions.into_iter().collect()
    }
}

fn main() {}
