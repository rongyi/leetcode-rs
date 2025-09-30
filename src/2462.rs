struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};
impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let mut costs = costs.into_iter();
        let candidates: usize = candidates as usize;
        let mut heap: BinaryHeap<(Reverse<i32>, bool)> = BinaryHeap::new();
        for v in costs.by_ref().take(candidates) {
            heap.push((Reverse(v), true));
        }
        for v in costs.by_ref().rev().take(candidates) {
            heap.push((Reverse(v), false));
        }
        let mut ret = 0;

        for _ in 0..k {
            let (Reverse(cur), is_front) = heap.pop().unwrap();
            ret += cur as i64;
            if let Some(next) = if is_front {
                costs.next()
            } else {
                costs.next_back()
            } {
                heap.push((Reverse(next), is_front));
            }
        }

        ret
    }
}

fn main() {
    let a = vec![1, 2, 3];
}
