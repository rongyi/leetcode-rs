struct Solution;

impl Solution {
    pub fn super_egg_drop(K: i32, n: i32) -> i32 {
        // K个鸡蛋 N个step所能测出的最大高度
        let mut dp = vec![vec![0; K as usize + 1]; n as usize + 1];
        let mut m = 0;

        while dp[m as usize][K as usize] < n {
            m += 1;
            for k in 1..=K {
                // 分鸡蛋碎了和没碎两种情况
                // if egg break, we can use (k-1) eggs and (s-1) to detect with Q(s- 1, k-1)
                // if egg isn't broken, we can use k eggs and (s-1) step to detech with Q(s - 1, k)
                // So, Q(s, k) = 1 + Q(s - 1, k) + Q(s - 1, k-1)
                dp[m as usize][k as usize] =
                    dp[(m - 1) as usize][(k - 1) as usize] + dp[(m - 1) as usize][k as usize] + 1;
            }
        }

        m
    }
}

fn main() {}
