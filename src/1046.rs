struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap: BinaryHeap<i32> = stones.into_iter().collect();

        while !heap.is_empty() {
            let big = heap.pop().unwrap();
            if let Some(small) = heap.pop() {
                if small != big {
                    heap.push(big - small);
                }
            } else {
                return big;
            }
        }

        0
    }
}

fn main() {}
