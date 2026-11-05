struct Solution;

impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut nums_with_bounds = vec![1; n + 2];
        nums_with_bounds[1..n + 1].clone_from_slice(&nums);
        let mut dp = vec![vec![0; n + 2]; n + 2];

        // bottom up
        for len in 1..=n {
            for i in 1..n + 2 - len {
                let j = i + len - 1;

                for k in i..=j {
                    dp[i][j] = dp[i][j].max(
                        nums_with_bounds[i - 1] * nums_with_bounds[k] * nums_with_bounds[j + 1]
                            + dp[i][k - 1]
                            + dp[k + 1][j],
                    );
                }
            }
        }

        dp[1][n]
    }
}

fn main() {}
