struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn mice_and_cheese(reward1: Vec<i32>, reward2: Vec<i32>, mut k: i32) -> i32 {
        let sz = reward1.len();
        let mut ret = 0;
        let mut pq: BinaryHeap<i32> = BinaryHeap::new();
        for i in 0..sz {
            pq.push(reward1[i] - reward2[i]);
            ret += reward2[i];
        }
        while k > 0 {
            ret += pq.pop().unwrap();
            k -= 1;
        }

        ret
    }
}

fn main() {}
