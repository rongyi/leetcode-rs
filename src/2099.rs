struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn max_subsequence(mut nums: Vec<i32>, mut k: i32) -> Vec<i32> {
        let mut num_wit_idx: BinaryHeap<(i32, usize)> =
            nums.into_iter().enumerate().map(|x| (x.1, x.0)).collect();
        let mut topk: Vec<(i32, usize)> = Vec::new();
        while k > 0 {
            topk.push(num_wit_idx.pop().unwrap());

            k -= 1;
        }
        topk.sort_by_key(|x| x.1);

        topk.into_iter().map(|x| x.0).collect()
    }
}

fn main() {}
