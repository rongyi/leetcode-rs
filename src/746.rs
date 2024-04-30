#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(mut cost: Vec<i32>) -> i32 {
        cost.push(0);
        let sz = cost.len();
        // cost to reach index i, push 0 means the top mountain
        let mut dp = vec![0; sz];
        dp[0] = cost[0];
        dp[1] = cost[1];

        for i in 2..sz {
            dp[i] = dp[i - 1].min(dp[i - 2]);
            dp[i] += cost[i];
        }

        dp[sz - 1]
    }
}

fn main() {}
