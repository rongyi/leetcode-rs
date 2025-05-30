struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        let mut max_heap = BinaryHeap::new();
        for pile in piles {
            max_heap.push(pile);
        }
        for _ in 0..k {
            if let Some(pile) = max_heap.pop() {
                max_heap.push(pile - pile / 2);
            }
        }
        max_heap.into_iter().sum()
    }
}

fn main() {}
