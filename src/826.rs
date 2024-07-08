#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_profit_assignment(
        difficulty: Vec<i32>,
        profit: Vec<i32>,
        mut worker: Vec<i32>,
    ) -> i32 {
        let mut dp: Vec<(i32, i32)> = Vec::new();
        for (&diff, &pft) in difficulty.iter().zip(profit.iter()) {
            dp.push((diff, pft));
        }
        dp.sort();
        worker.sort();
        let mut sum = 0;
        let mut max_pft = 0;
        let mut i = 0;
        let sz = difficulty.len();

        for &w in worker.iter() {
            // yeah, this worker *can* finish this job
            while i < sz && w >= dp[i].0 {
                max_pft = max_pft.max(dp[i].1);
                i += 1;
            }

            sum += max_pft;
        }

        sum
    }
}

fn main() {}
