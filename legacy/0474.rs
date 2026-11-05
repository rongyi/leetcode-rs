struct Solution;

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let len = strs.len();
        let mut dp = vec![vec![vec![0; n + 1]; m + 1]; len + 1];
        for i in 1..=len {
            for j in 0..=m {
                for k in 0..=n {
                    let cur = &strs[i - 1];
                    let ones = cur.chars().filter(|&c| c == '1').count();
                    let zeros = cur.len() - ones;
                    let mut cur_ret = dp[i - 1][j][k];
                    if zeros <= j && ones <= k {
                        cur_ret = cur_ret.max(dp[i - 1][j - zeros][k - ones] + 1);
                    }
                    dp[i][j][k] = cur_ret;
                }
            }
        }

        dp[len][m][n]
    }
}

fn main() {
    let mut a = vec![1, 1, 1, 1, 2];
    let b = a.iter().filter(|&&i| i == 1).count();
    println!("{:?}", b);
}
