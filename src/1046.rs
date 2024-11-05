struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap: BinaryHeap<i32> = stones.into_iter().collect();

        while !heap.is_empty() {
            let big = heap.pop().unwrap();
            let small = heap.pop();
            if small.is_none() {
                return big;
            }
            let small = small.unwrap();
            if small != big {
                heap.push(big - small);
            }
        }

        0
    }
}

fn main() {
    let mut heap = BinaryHeap::new();
    heap.push(1);
    heap.push(2);
    let cur = heap.pop().unwrap();
    println!("{}", cur);
}
