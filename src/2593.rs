struct Solution;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};
impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        let mut visited: HashSet<usize> = HashSet::new();
        let mut pq: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
        for (i, &v) in nums.iter().enumerate() {
            pq.push(Reverse((v, i)));
        }
        let mut acc = 0;
        while let Some(Reverse((val, idx))) = pq.pop() {
            if visited.contains(&idx) {
                continue;
            }
            visited.insert(idx);
            visited.insert(idx + 1);
            if idx > 0 {
                visited.insert(idx - 1);
            }
            acc += val as i64;
        }

        acc
    }
}

fn main() {}
