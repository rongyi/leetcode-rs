#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_difficulty(jobs: Vec<i32>, d: i32) -> i32 {
        let sz = jobs.len();
        let d = d as usize;
        // pigeon hole, even one job at a day can not satify d
        if d > sz {
            return -1;
        }
        // first, we need to calculate the diffcluty of range i..=j
        let mut difficlties = vec![vec![0; sz]; sz];
        // range of length 1
        for i in 0..sz {
            difficlties[i][i] = jobs[i];
        }
        // range length > 1
        for len in 2..=sz {
            for i in 0..=sz - len {
                let j = i + len - 1;
                difficlties[i][j] = difficlties[i][j - 1].max(jobs[j]);
            }
        }

        // dp[i][j] finish j jobs in first i days
        let mut dp = vec![vec![i32::MAX; sz]; d];
        // fill the firt row, i.e. finish all job in just one day
        for j in 0..sz {
            dp[0][j] = difficlties[0][j];
        }
        // calculate the rest
        for i in 1..d {
            // first i need to finish i jobs at least
            for j in i..sz {
                let mut opt = i32::MAX;
                for k in i - 1..j {
                    let cur = dp[i - 1][k] + difficlties[k + 1][j];
                    opt = opt.min(cur);
                }
                dp[i][j] = opt;
            }
        }

        if dp[d - 1][sz - 1] == i32::MAX {
            -1
        } else {
            dp[d - 1][sz - 1]
        }
    }
}

fn main() {}
