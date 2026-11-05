struct Solution;

impl Solution {
    pub fn merge_stones(stones: Vec<i32>, k: i32) -> i32 {
        let n = stones.len();
        let k = k as usize;

        // Check if merging down to a single pile is mathematically possible
        if (n - 1) % (k - 1) != 0 {
            return -1;
        }

        // Prefix sums for O(1) range sum calculation
        let mut prefix_sum = vec![0; n + 1];
        for i in 0..n {
            prefix_sum[i + 1] = prefix_sum[i] + stones[i];
        }

        const INF: i32 = 1_000_000_000;
        // dp[i][j] = min cost to reduce stones[i..=j] into as few piles as possible
        let mut dp = vec![vec![INF; n]; n];

        for i in 0..n {
            dp[i][i] = 0;
        }

        for len in 2..=n {
            for i in 0..=(n - len) {
                let j = i + len - 1;

                // Step mid by (k - 1) because the left subproblem stones[i..=mid]
                // can only be reduced to 1 pile if its length satisfies (mid - i) % (k - 1) == 0
                for mid in (i..j).step_by(k - 1) {
                    dp[i][j] = dp[i][j].min(dp[i][mid] + dp[mid + 1][j]);
                }

                // If range i..=j can be fully collapsed into 1 pile, add the sum of all elements
                if (len - 1) % (k - 1) == 0 {
                    dp[i][j] += prefix_sum[j + 1] - prefix_sum[i];
                }
            }
        }

        dp[0][n - 1]
    }
}

fn main() {}
