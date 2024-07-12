#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        // 游戏停止的区间是[k, k - 1 + max_pts]
        // 拿k-1游戏继续然后取了个最大值
        if k == 0 || n >= k + max_pts {
            return 1.0;
        }
        let mut dp = vec![0.0; n as usize + 1];
        dp[0] = 1.0;

        let mut wsum = 1.0;
        let mut ret = 0.0;
        for i in 1..=(n as usize) {
            dp[i] = wsum / max_pts as f64;
            if i < k as usize {
                wsum += dp[i];
            } else {
                ret += dp[i];
            }
            if (i as i32 - max_pts) >= 0 {
                wsum -= dp[i - max_pts as usize];
            }
        }

        ret
    }
}

fn main() {}
