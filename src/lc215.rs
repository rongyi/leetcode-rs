struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut pq = BinaryHeap::new();

        for num in nums.into_iter() {
            pq.push(num);
        }
        for _ in 0..k - 1 {
            pq.pop();
        }

        pq.pop().unwrap()
    }
}

fn main() {
    let mut h = BinaryHeap::new();
    h.push(10);
    h.push(9);
    println!("{}", h.pop().unwrap());
}
