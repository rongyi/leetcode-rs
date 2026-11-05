struct Solution;

impl Solution {
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; n as usize + 1]; min_profit as usize + 1];
        dp[0][0] = 1;
        let mut ret = 0;
        let m = 1e9 as i32 + 7;

        for k in 0..group.len() {
            // 干当前的这个坏事，收益 p，需要 g 个人总共
            let g = group[k];
            let p = profit[k];
            // 串起来,之前干坏事已有的收益 0..=min_profit都有可能，防止重复累加
            for i in (0..=min_profit).rev() {
                // 还剩下多少个人可以用
                let mut j = n - g;
                // 用起来
                while j >= 0 {
                    // 多了也没必要，所以就卡在minprofit就可以了
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
