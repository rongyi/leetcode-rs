#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_distance(mut houses: Vec<i32>, k: i32) -> i32 {
        let sz = houses.len();
        let k = k as usize;

        houses.sort_unstable();

        // Precompute the cost of placing one mailbox for a range of houses
        let mut cost = vec![vec![0; sz]; sz];
        for i in 0..sz {
            for j in i..sz {
                let median = houses[(i + j) / 2];
                let mut total = 0;
                for p in i..=j {
                    total += (houses[p] - median).abs();
                }
                cost[i][j] = total;
            }
        }
        // Dynamic programming table: dp[i][j] = minimum distance for first i houses using j mailboxes
        let mut dp = vec![vec![i32::MAX; k + 1]; sz + 1];
        dp[0][0] = 0;

        for i in 1..=sz {
            for j in 1..=k {
                for p in 0..i {
                    if dp[p][j - 1] != i32::MAX {
                        dp[i][j] = dp[i][j].min(dp[p][j - 1] + cost[p][i - 1]);
                    }
                }
            }
        }

        dp[sz][k]
    }
}

fn main() {}
