struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn minimize_the_difference(mat: Vec<Vec<i32>>, target: i32) -> i32 {
        let m = mat.len();
        let n = mat[0].len();

        // Initialize DP with a HashSet (starting with sum 0)
        let mut dp = HashSet::new();
        dp.insert(0);

        for i in 0..m {
            let mut new_dp = HashSet::new();
            let row = &mat[i];

            // For each sum in the current DP, add each element in the current row
            for &sum in &dp {
                for &num in row {
                    new_dp.insert(sum + num);
                }
            }

            // Early pruning: Keep only sums that could potentially minimize the difference
            let min_diff_so_far = new_dp
                .iter()
                .map(|&s| (s - target).abs())
                .min()
                .unwrap_or(i32::MAX);

            // Filter sums that are too large to be useful
            let max_reasonable_sum = target + min_diff_so_far;
            dp = new_dp
                .into_iter()
                .filter(|&s| s <= max_reasonable_sum)
                .collect();
        }

        // Find the sum with the smallest absolute difference to target
        dp.into_iter()
            .map(|s| (s - target).abs())
            .min()
            .unwrap_or(0)
    }
}

fn main() {}
