struct Solution;

impl Solution {
    pub fn max_taxi_earnings(n: i32, mut rides: Vec<Vec<i32>>) -> i64 {
        rides.sort_unstable();
        let n = n as usize;
        let mut dp = vec![0i64; n + 1];
        // index in rides
        let mut j = 0;

        for i in 1..=n {
            // dont pick current client
            dp[i] = dp[i].max(dp[i - 1]);
            // pick current client
            // split at current pos
            while j < rides.len() && rides[j][0] == i as i32 {
                dp[rides[j][1] as usize] = dp[rides[j][1] as usize]
                    .max(dp[i] + (rides[j][1] - rides[j][0] + rides[j][2]) as i64);

                j += 1;
            }
        }

        dp[n]
    }
}
fn main() {}
