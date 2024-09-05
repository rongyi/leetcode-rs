struct Solution;

impl Solution {
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; n as usize + 1]; min_profit as usize + 1];
        dp[0][0] = 1;
        let mut ret = 0;
        let m = 1e9 as i32 + 7;

        for k in 0..group.len() {
            let g = group[k];
            let p = profit[k];
            for i in (0..=min_profit).rev() {
                let mut j = n - g;
                while j >= 0 {
                    dp[(i + p).min(min_profit) as usize][(j + g) as usize] = (dp
                        [(i + p).min(min_profit) as usize][(j + g) as usize]
                        + dp[i as usize][j as usize])
                        % m;
                    j -= 1;
                }
            }
        }

        for &x in dp[min_profit as usize].iter() {
            ret = (ret + x) % m;
        }

        ret
    }
}

fn main() {}
