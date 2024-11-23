struct Solution;

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let sz = piles.len();
        let mut suffix_sum = vec![0; sz];

        suffix_sum[sz - 1] = piles[sz - 1];
        for i in (0..sz - 1).rev() {
            suffix_sum[i] = suffix_sum[i + 1] + piles[i];
        }

        let mut dp = vec![vec![-1; sz + 1]; sz];

        fn dfs(i: usize, m: usize, suffix_sum: &Vec<i32>, dp: &mut Vec<Vec<i32>>) -> i32 {
            if i >= suffix_sum.len() {
                return 0;
            }
            if dp[i][m] != -1 {
                return dp[i][m];
            }
            let mut ret = 0;
            let total = suffix_sum[i];
            for x in 1..=2 * m {
                if i + x > suffix_sum.len() {
                    break;
                }
                let oppnent = dfs(i + x, x.max(m), suffix_sum, dp);
                ret = ret.max(total - oppnent);
            }

            dp[i][m] = ret;
            ret
        }

        dfs(0, 1, &suffix_sum, &mut dp)
    }
}

fn main() {}
