#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn minimum_incompatibility(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let subset_size = n / k as usize;

        // If we can't evenly divide the array, return -1
        if n % k as usize != 0 {
            return -1;
        }

        // Create a frequency map to check for elements that appear more than k times
        let mut freq = vec![0; 17]; // nums[i] is between 1 and 16
        for &num in &nums {
            freq[num as usize] += 1;
            if freq[num as usize] > k {
                return -1; // If any number appears more than k times, impossible to partition
            }
        }

        // Precompute the incompatibility of all valid subsets
        // We'll use bit manipulation for subsets - each bit represents whether an element is in the subset
        let full_set: usize = (1 << n) - 1;
        let mut dp = vec![i32::MAX; 1 << n];
        let mut incompatibility = vec![i32::MAX; 1 << n];

        // Calculate incompatibility for all valid subsets of size subset_size
        for mask in 1..=full_set {
            if mask.count_ones() != subset_size as u32 {
                continue;
            }

            let mut subset = vec![];
            let mut used = vec![false; 17];
            let mut valid = true;

            for i in 0..n {
                if (mask & (1 << i)) != 0 {
                    let num = nums[i] as usize;
                    if used[num] {
                        valid = false;
                        break;
                    }
                    used[num] = true;
                    subset.push(nums[i]);
                }
            }

            if valid {
                subset.sort();
                let min_val = subset[0];
                let max_val = subset[subset_size - 1];
                incompatibility[mask] = max_val - min_val;
            }
        }

        // Initialize DP array
        dp[0] = 0;

        // Dynamic programming to find minimum total incompatibility
        for mask in 0..full_set {
            if dp[mask] == i32::MAX {
                continue;
            }

            // Find all valid subsets to extend the current state
            let remaining = full_set ^ mask;
            let mut submask = remaining;

            while submask > 0 {
                // Check if submask has exactly subset_size elements
                if submask.count_ones() == subset_size as u32
                    && incompatibility[submask] != i32::MAX
                {
                    dp[mask | submask] =
                        std::cmp::min(dp[mask | submask], dp[mask] + incompatibility[submask]);
                }

                // Get the next submask of the remaining elements
                submask = (submask - 1) & remaining;
            }
        }

        if dp[full_set] == i32::MAX {
            -1
        } else {
            dp[full_set]
        }
    }
}

fn main() {}
