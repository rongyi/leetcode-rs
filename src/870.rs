struct Solution;

use std::collections::BinaryHeap;
use std::collections::VecDeque;

impl Solution {
    pub fn advantage_count(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let n = nums1.len();
        let mut nums1_sorted = nums1.clone();
        nums1_sorted.sort_unstable();
        let mut available: VecDeque<i32> = nums1_sorted.into_iter().collect();

        // Use max-heap for nums2 to process largest values first
        let mut heap = BinaryHeap::new();
        for (i, &val) in nums2.iter().enumerate() {
            heap.push((val, i));
        }

        let mut result = vec![0; n];

        while let Some((target, idx)) = heap.pop() {
            // For the largest remaining target, we need the largest possible
            // element that can beat it, or sacrifice the smallest
            if let Some(&largest) = available.back() {
                if largest > target {
                    // Use the largest available to beat the largest target
                    result[idx] = available.pop_back().unwrap();
                } else {
                    // Can't beat it, sacrifice the smallest
                    result[idx] = available.pop_front().unwrap();
                }
            }
        }

        result
    }
}

fn main() {}
