struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut q: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        let k = k as usize;
        for num in nums.into_iter() {
            q.push(Reverse(num));
            if q.len() > k {
                q.pop();
            }
        }
        q.pop().unwrap().0
    }
}

fn main() {}
