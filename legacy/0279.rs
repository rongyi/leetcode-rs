struct Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = 0;
        for i in 1..=n {
            for j in 1.. {
                if j * j > i {
                    break;
                }
                dp[i] = dp[i].min(dp[i - j * j] + 1);
            }
        }

        dp[n]
    }
}

fn main() {
    unimplemented!();
}
