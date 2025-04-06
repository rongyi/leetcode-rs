#![allow(dead_code)]

struct Solution;

use std::cmp::{min, Ordering};

impl Solution {
    pub fn min_absolute_sum_diff(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        // Create a sorted copy of nums1 for binary search
        let mut sorted_nums1 = nums1.clone();
        sorted_nums1.sort();

        let mut total_diff: i64 = 0;
        let mut max_diff_reduction: i64 = 0;

        for i in 0..nums1.len() {
            let curr_diff = (nums1[i] - nums2[i]).abs() as i64;
            total_diff += curr_diff;

            // Find the closest value in nums1 to nums2[i]
            let idx = binary_search(&sorted_nums1, nums2[i]);

            // Check the element found by binary search
            // Here we use the binary search result to find the element in nums1 that is closest to nums2[i].
            // If we replace nums1[i] with sorted_nums1[idx], we can potentially reduce the absolute difference.
            // We'll calculate this new potential difference and see how much we could reduce the total sum by.
            let potential_diff = (sorted_nums1[idx] - nums2[i]).abs() as i64;
            let diff_reduction = curr_diff - potential_diff;
            max_diff_reduction = max_diff_reduction.max(diff_reduction);

            // If idx > 0, also check the element before it
            if idx > 0 {
                let potential_diff = (sorted_nums1[idx - 1] - nums2[i]).abs() as i64;
                let diff_reduction = curr_diff - potential_diff;
                max_diff_reduction = max_diff_reduction.max(diff_reduction);
            }
        }

        // Return the result after modulo
        ((total_diff - max_diff_reduction) % MOD) as i32
    }
}

// Custom binary search to find the index of the closest value to target
fn binary_search(arr: &Vec<i32>, target: i32) -> usize {
    let mut left = 0;
    let mut right = arr.len() - 1;

    if target <= arr[0] {
        return 0;
    }

    if target >= arr[right] {
        return right;
    }

    while left < right {
        let mid = left + (right - left) / 2;

        match arr[mid].cmp(&target) {
            Ordering::Equal => return mid,
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid,
        }
    }

    // If target is closer to arr[left-1], return left-1, otherwise return left
    if left > 0 && target - arr[left - 1] < arr[left] - target {
        left - 1
    } else {
        left
    }
}

fn main() {}
