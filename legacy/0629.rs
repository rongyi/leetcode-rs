struct Solution;

impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        let prime = 1e9 as i64 + 7;

        // dp[n][k] denotes the number of arrays that have k inverse pairs for array composed of 1 to n
        // we can establish the recursive relationship between dp[n][k] and dp[n-1][i]:

        // if we put n as the last number then all the k inverse pair should come from the first n-1 numbers
        // if we put n as the second last number then there's 1 inverse pair involves n so the rest k-1 comes from the first n-1 numbers
        // ...
        // if we put n as the first number then there's n-1 inverse pairs involve n so the rest k-(n-1) comes from the first n-1 numbers

        // dp[n][k] = dp[n-1][k]+dp[n-1][k-1]+dp[n-1][k-2]+...+dp[n-1][k+1-n+1]+dp[n-1][k-n+1]
        let mut dp = vec![vec![0i64; k as usize + 1]; n as usize + 1];
        dp[0][0] = 1;
        for i in 1..=n as usize {
            for j in 0..=k as usize {
                // upper bound for m is min(j, i - 1)
                // i - 1 is the last case, put largest num in front to make i - 1 inverse array
                for m in 0..=j.min(i - 1) {
                    dp[i][j] += dp[i - 1][j - m];
                }
                dp[i][j] %= prime;
            }
        }

        dp[n as usize][k as usize] as i32
    }
}

fn main() {}
