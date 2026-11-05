struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![i32::MAX; (amount + 1) as usize];
        dp[0] = 0;
        for i in 1..=amount {
            for &c in &coins {
                if i - c >= 0 && dp[(i - c) as usize] != i32::MAX {
                    dp[i as usize] = dp[i as usize].min(dp[(i - c) as usize] + 1);
                }
            }
        }
        if dp[amount as usize] == i32::MAX {
            return -1;
        }
        return dp[amount as usize];
    }
}

fn main() {}
