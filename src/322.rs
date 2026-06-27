struct Solution;

impl Solution {
    /// LeetCode 322: Coin Change
    ///
    /// Returns the fewest number of coins needed to make `amount`.
    /// Returns -1 if the amount cannot be made up by any combination.
    ///
    /// # Approach: DP (unbounded knapsack)
    ///
    /// dp[i] = minimum coins to make amount i.
    /// For each amount, try using each coin c:
    ///   dp[i] = min(dp[i], dp[i - c] + 1)
    pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
        // dp[i] = min coins to make amount i.
        // Initialize with i32::MAX as "unreachable" sentinel.
        let mut dp = vec![i32::MAX; amount as usize + 1];
        dp[0] = 0; // 0 takes 0 coins

        // Sort ascending so we can break early below.
        coins.sort_unstable();

        for cur_amount in 1..=amount {
            for &c in coins.iter() {
                if c <= cur_amount {
                    let prev = (cur_amount - c) as usize;
                    // Only try c if the remaining amount is reachable.
                    // Guard prevents i32::MAX + 1 overflow.
                    if dp[prev] != i32::MAX {
                        dp[cur_amount as usize] = dp[cur_amount as usize].min(dp[prev] + 1);
                    }
                } else {
                    // Coins are sorted — all remaining coins are also too large.
                    break;
                }
            }
        }

        if dp[amount as usize] == i32::MAX {
            return -1;
        }

        dp[amount as usize]
    }
}

fn main() {}
