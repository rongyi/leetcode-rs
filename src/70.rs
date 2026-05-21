struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let sz = n as usize;
        let mut dp = vec![0; sz + 1];
        dp[0] = 1;
        dp[1] = 1;

        for i in 2..=sz {
            dp[i] = dp[i - 1] + dp[i - 2];
        }

        dp[sz]
    }
}

fn main() {}
