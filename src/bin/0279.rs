struct Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let sz = n as usize;
        let mut dp = vec![i32::MAX; sz + 1];
        dp[0] = 0;

        for i in 1..=sz {
            let mut j = 1;
            while j * j <= i {
                dp[i] = dp[i].min(dp[i - j * j] + 1);
                j += 1;
            }
        }

        dp[sz]
    }
}

fn main() {}
