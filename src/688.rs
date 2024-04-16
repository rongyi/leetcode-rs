#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
        // only use f64 can pass
        let mut dp = vec![vec![vec![-1.0; n as usize]; n as usize]; (k + 1) as usize];

        Self::recur(&mut dp, n, k, row, column) / 8f64.powf(k as f64) // total sample space
    }

    fn recur(dp: &mut Vec<Vec<Vec<f64>>>, n: i32, k: i32, r: i32, c: i32) -> f64 {
        if r < 0 || r >= n || c < 0 || c >= n {
            return 0.0;
        }

        if k == 0 {
            return 1.0;
        }

        if dp[k as usize][r as usize][c as usize] != -1.0 {
            return dp[k as usize][r as usize][c as usize];
        }

        dp[k as usize][r as usize][c as usize] = 0.0;

        for i in -2..=2 {
            if i == 0 {
                continue;
            }

            dp[k as usize][r as usize][c as usize] +=
                Self::recur(dp, n, k - 1, r + i, c + (3 - i.abs()))
                    + Self::recur(dp, n, k - 1, r + i, c - (3 - i.abs()));
        }

        dp[k as usize][r as usize][c as usize]
    }
}

fn main() {
    let a = 8f64.powf(30f64);
    println!("{:?}", a);
}
