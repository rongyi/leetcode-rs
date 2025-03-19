#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn kth_smallest_path(destination: Vec<i32>, k: i32) -> String {
        let rows = destination[0] as usize;
        let cols = destination[1] as usize;
        let mut k = k as usize;

        // Total number of steps needed: rows + cols
        // We need to choose which of these steps are Horizontal ('H')
        // The rest will be Vertical ('V')
        let h_steps = cols;
        let v_steps = rows;
        let total_steps = h_steps + v_steps;

        // Calculate binomial coefficients for all possible positions
        // dp[i][j] represents the number of ways to choose j items from i items
        let mut dp = vec![vec![0; total_steps + 1]; total_steps + 1];
        dp[0][0] = 1;

        for i in 1..=total_steps {
            dp[i][0] = 1;
            for j in 1..=i {
                dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j];
            }
        }

        let mut result = String::new();
        let mut remaining_h = h_steps;
        let mut remaining_v = v_steps;

        // Construct the path step by step
        for _step in 0..total_steps {
            // If all remaining steps must be vertical (no horizontal left)
            if remaining_h == 0 {
                result.push('V');
                remaining_v -= 1;
                continue;
            }

            // If all remaining steps must be horizontal (no vertical left)
            if remaining_v == 0 {
                result.push('H');
                remaining_h -= 1;
                continue;
            }

            // Count ways to reach destination if we go horizontal at this step
            // This is calculated using binomial coefficient C(remaining_steps-1, remaining_h-1)
            let ways_with_h = dp[remaining_h + remaining_v - 1][remaining_h - 1];

            // If k <= ways_with_h, then we go horizontal; otherwise, we go vertical
            if k <= ways_with_h {
                result.push('H');
                remaining_h -= 1;
            } else {
                result.push('V');
                remaining_v -= 1;
                k -= ways_with_h;
            }
        }

        result
    }
}

fn main() {}
