#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn soup_servings(n: i32) -> f64 {
        if n >= 5000 {
            return 1.0;
        }
        let mut dp = vec![vec![0.0; n as usize + 1]; n as usize + 1];

        Self::dfs(n, n, &mut dp)
    }

    fn dfs(a: i32, b: i32, dp: &mut Vec<Vec<f64>>) -> f64 {
        if a <= 0 && b <= 0 {
            return 0.5;
        }
        if a <= 0 {
            return 1.0;
        }
        if b <= 0 {
            return 0.0;
        }
        if dp[a as usize][b as usize] != 0.0 {
            return dp[a as usize][b as usize];
        }

        let serve_a = [100, 75, 50, 25];
        let serve_b = [0, 25, 50, 75];

        dp[a as usize][b as usize] = 0.0;
        for i in 0..serve_a.len() {
            dp[a as usize][b as usize] += Self::dfs(a - serve_a[i], b - serve_b[i], dp);
        }

        dp[a as usize][b as usize] *= 0.25;

        dp[a as usize][b as usize]
    }
}

fn main() {}
