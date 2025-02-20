#![allow(dead_code)]

struct Solution;

impl Solution {
    const MOD: i64 = 1e9 as i64 + 7;
    pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
        let pizza: Vec<Vec<char>> = pizza.into_iter().map(|s| s.chars().collect()).collect();
        let m = pizza.len();
        let n = pizza[0].len();
        let mut dp = vec![vec![vec![-1; n]; m]; k as usize];
        let mut suffix_sum = vec![vec![0; n + 1]; m + 1];

        // 切完都是看右下角这块还是否有苹果可切，prefix_sum算的就是这右下角
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                suffix_sum[i][j] = suffix_sum[i + 1][j] + suffix_sum[i][j + 1]
                    - suffix_sum[i + 1][j + 1]
                    + (pizza[i][j] == 'A') as i64;
            }
        }

        Self::dfs(m, n, k - 1, 0, 0, &mut dp, &suffix_sum) as _
    }

    fn dfs(
        m: usize,
        n: usize,
        k: i32,
        x: usize,
        y: usize,
        dp: &mut Vec<Vec<Vec<i64>>>,
        suffix_sum: &Vec<Vec<i64>>,
    ) -> i64 {
        // 无苹果可切
        if suffix_sum[x][y] == 0 {
            return 0;
        }
        // 切完了，发现一个valid组合
        if k <= 0 {
            return 1;
        }
        if dp[k as usize][x][y] != -1 {
            return dp[k as usize][x][y];
        }
        let mut ret = 0;

        // 试试下刀的地方，切一遭走起！
        // 横着切
        for i in x + 1..m {
            if suffix_sum[x][y] - suffix_sum[i][y] > 0 {
                ret = (ret + Self::dfs(m, n, k - 1, i, y, dp, suffix_sum)) % Solution::MOD;
            }
        }

        // 竖着切
        for j in y + 1..n {
            if suffix_sum[x][y] - suffix_sum[x][j] > 0 {
                ret = (ret + Self::dfs(m, n, k - 1, x, j, dp, suffix_sum)) % Solution::MOD;
            }
        }

        dp[k as usize][x][y] = ret;

        ret
    }
}

fn main() {
    let input: Vec<String> = ["A..", "AAA", "..."]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let val = Solution::ways(input, 3);

    println!("{val}");
}
