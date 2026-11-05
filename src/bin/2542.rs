struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};
impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let nums1: Vec<i64> = nums1.into_iter().map(|v| v as i64).collect();
        let nums2: Vec<i64> = nums2.into_iter().map(|v| v as i64).collect();

        let mut nums1_with_idx: Vec<(i64, usize)> = nums1
            .into_iter()
            .enumerate()
            .map(|(idx, val)| (val, idx))
            .collect();
        nums1_with_idx.sort_by(|a, b| b.cmp(&a));
        let mut pq: BinaryHeap<Reverse<(i64, i64)>> = BinaryHeap::new();

        let mut ret = 0;
        let mut cur_sum = 0;

        let k = k as usize;
        for (val, idx) in nums1_with_idx.into_iter() {
            cur_sum += val;
            pq.push(Reverse((nums2[idx], val)));
            if pq.len() == k {
                let (lowest, val) = pq.pop().unwrap().0;
                ret = ret.max(cur_sum * lowest);
                cur_sum -= val;
            }
        }

        ret
    }
}

fn main() {}
