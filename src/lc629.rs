struct Solution;

impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        let prime = 1e9 as i64 + 7;
        let sum = n * (n - 1) / 2;
        if k > sum || k < 0 {
            return 0;
        }
        if k == 0 || k == sum {
            return 1;
        }

        // dp[n][k] denotes the number of arrays that have k inverse pairs for array composed of 1 to n
        // we can establish the recursive relationship between dp[n][k] and dp[n-1][i]:

        // if we put n as the last number then all the k inverse pair should come from the first n-1 numbers
        // if we put n as the second last number then there's 1 inverse pair involves n so the rest k-1 comes from the first n-1 numbers
        // ...
        // if we put n as the first number then there's n-1 inverse pairs involve n so the rest k-(n-1) comes from the first n-1 numbers

        // dp[n][k] = dp[n-1][k]+dp[n-1][k-1]+dp[n-1][k-2]+...+dp[n-1][k+1-n+1]+dp[n-1][k-n+1]
        let mut dp = vec![vec![0i64; k as usize + 1]; n as usize];
        dp[0][0] = 1;
        for i in 1..n as usize {
            for j in 0..=k as usize {
                let mut m = j as i32;
                while m >= 0 && m >= (j - i) as i32 {
                    dp[i][j] += dp[i - 1][m as usize];
                    m -= 1;
                }

                dp[i][j] %= prime;
            }
        }

        dp[n as usize - 1][k as usize] as i32
    }
}

fn main() {}
