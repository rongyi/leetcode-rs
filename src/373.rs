
pub struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
    /// LeetCode 373: Find K Pairs with Smallest Sums
    ///
    /// Given two sorted arrays, return the k pairs (u, v) with the
    /// smallest sums, where u ∈ nums1, v ∈ nums2.
    ///
    /// # Approach: Min-heap (BFS on the 2D sum matrix)
    ///
    /// Think of pairs as a grid: rows = nums1, cols = nums2.
    /// Sums increase going right or down (both arrays are sorted).
    ///
    /// 1. Seed the heap with the first column: (nums1[i] + nums2[0], i, 0)
    ///    for i in 0..min(k, len1).
    /// 2. Pop the smallest. Record the pair. Then advance one column:
    ///    push (nums1[i] + nums2[j+1], i, j+1) if j+1 < len2.
    /// 3. Repeat k times.
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let (n1, n2) = (nums1.len(), nums2.len());
        if n1 == 0 || n2 == 0 {
            return vec![];
        }

        let k = k as usize;
        let mut heap = BinaryHeap::new();
        let mut result = Vec::with_capacity(k);

        // Seed: first element of nums1 paired with each nums2[0].
        // Only need up to k seeds (more than k rows can't all produce top-k).
        for i in 0..n1.min(k) {
            heap.push(Reverse((nums1[i] + nums2[0], i, 0usize)));
        }

        while result.len() < k && !heap.is_empty() {
            let Reverse((_sum, i, j)) = heap.pop().unwrap();
            result.push(vec![nums1[i], nums2[j]]);

            // Advance to the next column for this row.
            if j + 1 < n2 {
                heap.push(Reverse((nums1[i] + nums2[j + 1], i, j + 1)));
            }
        }

        result
    }
}

fn main() {
    let tests = [
        (
            vec![1, 7, 11],
            vec![2, 4, 6],
            3,
            vec![vec![1, 2], vec![1, 4], vec![1, 6]],
        ),
        (
            vec![1, 1, 2],
            vec![1, 2, 3],
            2,
            vec![vec![1, 1], vec![1, 1]],
        ),
        (vec![1, 2], vec![3], 3, vec![vec![1, 3], vec![2, 3]]),
    ];

    for (nums1, nums2, k, expected) in &tests {
        let result = Solution::k_smallest_pairs(nums1.clone(), nums2.clone(), *k);
        println!(
            "{} nums1={:?} nums2={:?} k={} → {:?}",
            if result == *expected { "✓" } else { "✗" },
            nums1,
            nums2,
            k,
            result
        );
    }
}
