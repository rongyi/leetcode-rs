#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = multipliers.len();

        // dp[i][j] represents the maximum score when we have used i elements from the left
        // and j elements from the right, where i + j is the current operation
        let mut dp = vec![vec![0; m + 1]; m + 1];

        for op in 1..=m {
            // For operation 'op', we'll try all possible combinations of taking elements
            // from the left and right sides of the nums array.
            // If we've taken 'left' elements from the left side,
            // then we must have taken 'op - left' elements from the right side.
            for left in 0..=op {
                // "right" represents the number of elements we've taken from the right side
                // of the nums array so far. Since we have to take exactly 'op' elements in total,
                // and we've taken 'left' from the left side, the remaining (op - left) must
                // come from the right side.
                let right = op - left;

                if right > n || left > m {
                    continue;
                }

                // The right_index is calculated as (n - right), which gives us the index of
                // the element from the right side of the array.
                // When right = 1, we're taking the last element (index n-1)
                // When right = 2, we're taking the second last element (index n-2)
                // and so on...
                let right_index = n - right;

                // We can either take from the left or the right
                let mut max_score = std::i32::MIN;

                // Take from the left
                if left > 0 {
                    max_score =
                        max_score.max(dp[left - 1][right] + nums[left - 1] * multipliers[op - 1]);
                }

                // Take from the right
                if right > 0 {
                    max_score = max_score
                        .max(dp[left][right - 1] + nums[right_index] * multipliers[op - 1]);
                }

                // If this is the first operation, set the initial value
                if left == 0 && right == 0 {
                    max_score = 0;
                }

                dp[left][right] = max_score;
            }
        }

        // Find the maximum score among all possible ways to perform m operations
        let mut max_score = std::i32::MIN;
        for left in 0..=m {
            let right = m - left;
            if right <= n && left <= m {
                max_score = max_score.max(dp[left][right]);
            }
        }

        max_score
    }
}

fn main() {}
