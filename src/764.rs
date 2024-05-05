#![allow(dead_code)]


struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        // sparse matrix
        let mut matrix = HashSet::new();
        for hole in mines.into_iter() {
            matrix.insert(hole[0] * n + hole[1]);
        }

        let mut ret = 0;
        let mut cnt = 0;
        let mut dp = vec![vec![0; n as usize]; n as usize];

        for j in 0..n {
            cnt = 0;
            for i in 0..n {
                cnt = if matrix.contains(&(i * n + j)) {
                    0
                } else {
                    cnt + 1
                };
                dp[i as usize][j as usize] = cnt;
            }
            cnt = 0;
            for i in (0..n).rev() {
                cnt = if matrix.contains(&(i * n + j)) {
                    0
                } else {
                    cnt + 1
                };
                dp[i as usize][j as usize] = cnt.min(dp[i as usize][j as usize]);
            }
        }

        for i in 0..n {
            cnt = 0;
            for j in 0..n {
                cnt = if matrix.contains(&(i * n + j)) {
                    0
                } else {
                    cnt + 1
                };
                dp[i as usize][j as usize] = dp[i as usize][j as usize].min(cnt);
            }

            cnt = 0;
            for j in (0..n).rev() {
                cnt = if matrix.contains(&(i * n + j)) {
                    0
                } else {
                    cnt + 1
                };
                dp[i as usize][j as usize] = cnt.min(dp[i as usize][j as usize]);

                ret = ret.max(dp[i as usize][j as usize]);
            }
        }

        ret
    }
}

fn main() {}
