struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(mut cost: Vec<i32>) -> i32 {
        let sz = cost.len();
        // cost to reach index i, push 0 means the top mountain
        let mut dp = vec![0; sz + 1];

        for i in 2..=sz {
            dp[i] = (dp[i - 1] + cost[i - 1]).min(dp[i - 2] + cost[i - 2]);
        }

        dp[sz]
    }
}

fn main() {}
