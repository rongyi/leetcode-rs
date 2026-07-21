struct Solution;

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        // dp[i][j]  prefix: i coins sum to amount: j
        if amount == 0 {
            return 1;
        }
        let amount = amount as usize;
        let sz = coins.len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; amount + 1]; sz + 1];
        dp[0][0] = 1;

        for i in 1..=sz {
            // empty case
            dp[i][0] = 1;
            for j in 1..=amount {
                // for last i - 1 index coin, we take it or not
                // if we dont take it
                dp[i][j] = dp[i - 1][j];
                // if we take it, and we need amout j >= coinst[i - 1]
                if j as i32 >= coins[i - 1] {
                    dp[i][j] += dp[i][j - coins[i - 1] as usize];
                }
            }
        }

        dp[sz][amount]
    }
}

fn main() {}
