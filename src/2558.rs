
struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, mut k: i32) -> i64 {
        let mut pq: BinaryHeap<i64> = gifts.into_iter().map(|v| v as i64).collect();
        while k > 0 {
            let cur = pq.pop().unwrap();
            let update = (cur as f64).sqrt().floor() as i64;
            pq.push(update);
            k -= 1
        }
        pq.into_iter().sum()
    }
}

fn main() {}
