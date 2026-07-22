struct Solution;

impl Solution {
    /// LeetCode 540: Single Element in a Sorted Array
    ///
    /// Every element appears twice except one. Find it in O(log n).
    ///
    /// # Key insight: pair positions shift after the singleton
    ///
    /// Before singleton:  pairs at (even, odd)  →  nums[0]==nums[1], nums[2]==nums[3]
    /// After singleton:   pairs shift to (odd, even)
    ///
    /// Binary search: for a given `mid`, check which side the singleton is on.
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (0, nums.len() - 1);

        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            // Force mid to be even — compare with the paired element.
            let pair = if mid % 2 == 1 { mid - 1 } else { mid };

            if nums[pair] == nums[pair + 1] {
                // Pair is intact → singleton is after this pair.
                lo = pair + 2;
            } else {
                // Pair is broken → singleton is at or before mid.
                hi = mid;
            }
        }

        nums[lo]
    }
}

fn main() {
    let tests = [
        (vec![1, 1, 2, 3, 3, 4, 4, 8, 8], 2),
        (vec![3, 3, 7, 7, 10, 11, 11], 10),
        (vec![1], 1),
        (vec![1, 1, 2], 2),
    ];

    for (nums, expected) in &tests {
        let result = Solution::single_non_duplicate(nums.clone());
        println!(
            "{} nums={:?} → {} (expected {})",
            if result == *expected { "✓" } else { "✗" },
            nums,
            result,
            expected
        );
    }
}
