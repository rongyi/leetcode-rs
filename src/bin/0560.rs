
struct Solution;

use std::collections::HashMap;
impl Solution {
    /// LeetCode 560: Subarray Sum Equals K
    ///
    /// Prefix sum + HashMap. At each step, look up how many times
    /// (sum - k) has appeared as a prefix sum before.
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut prefix_count = HashMap::new();
        prefix_count.insert(0, 1); // empty prefix sums to 0

        let (mut sum, mut ans) = (0, 0);
        for num in nums {
            sum += num;
            // Each prior prefix of value (sum - k) gives a subarray ending here.
            ans += prefix_count.get(&(sum - k)).unwrap_or(&0);
            *prefix_count.entry(sum).or_insert(0) += 1;
        }

        ans
    }
}

fn main() {
    let tests = [
        (vec![1, 1, 1], 2, 2),
        (vec![1, 2, 3], 3, 2),
        (vec![-1, -1, 1], 0, 1),
    ];

    for (nums, k, expected) in &tests {
        let result = Solution::subarray_sum(nums.clone(), *k);
        println!(
            "{} nums={:?} k={} → {} (expected {})",
            if result == *expected { "✓" } else { "✗" },
            nums,
            k,
            result,
            expected
        );
    }
}
