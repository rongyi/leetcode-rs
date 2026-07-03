
struct Solution;

use std::i32;
impl Solution {
    /// LeetCode 375: Guess Number Higher or Lower II
    ///
    /// You guess a number in [1, n]. Wrong guess at x costs $x.
    /// Find the minimum money needed to **guarantee** a win (worst-case).
    ///
    /// # Approach: Minimax DP
    ///
    /// For range [l, r], if you guess x:
    ///   - Correct → cost 0
    ///   - Too high → pay x + cost for [l, x-1]
    ///   - Too low  → pay x + cost for [x+1, r]
    ///
    /// Worst case = max(left_cost, right_cost).
    /// You choose x to minimize that worst case:
    ///   dp[l][r] = min_{x∈[l,r]} (x + max(dp[l][x-1], dp[x+1][r]))

    // ── Top-down (memoization) ──

    pub fn get_money_amount(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![0; n + 1]; n + 1];
        Self::recur(1, n, &mut dp)
    }

    fn recur(l: usize, r: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
        // hit this bitch
        if l >= r {
            return 0;
        }
        if dp[l][r] != 0 {
            return dp[l][r];
        }
        let mut cur = i32::MAX;

        for i in l..=r {
            cur = cur.min(
                i as i32 + std::cmp::max(Self::recur(l, i - 1, dp), Self::recur(i + 1, r, dp)),
            );
        }

        dp[l][r] = cur;
        cur
    }
}

fn main() {}
