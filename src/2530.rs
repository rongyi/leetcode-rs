struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn max_kelements(nums: Vec<i32>, mut k: i32) -> i64 {
        let mut pq: BinaryHeap<i32> = nums.into_iter().collect();
        let mut sum = 0;
        while k > 0 {
            match pq.pop() {
                Some(val) => {
                    sum += val as i64;
                    pq.push((val + 2) / 3);
                }
                None => break,
            }
            k -= 1;
        }
        sum
    }
}

fn main() {}
