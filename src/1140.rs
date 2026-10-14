struct Solution;

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let sz = piles.len();
        let mut suffix_sum = vec![0; sz + 1];

        for i in (0..sz).rev() {
            suffix_sum[i] = suffix_sum[i + 1] + piles[i];
        }

        let mut memo = vec![vec![-1; sz + 1]; sz];

        fn dfs(
            i: usize,
            m: usize,
            piles: &[i32],
            suffix_sum: &Vec<i32>,
            memo: &mut Vec<Vec<i32>>,
        ) -> i32 {
            if i + 2 * m >= piles.len() {
                return suffix_sum[i];
            }

            if memo[i][m] != -1 {
                return memo[i][m];
            }

            let mut ret = 0;
            let total = suffix_sum[i];

            for x in 1..=2 * m {
                if i + x > suffix_sum.len() {
                    break;
                }
                let oppnent = dfs(i + x, x.max(m), piles, suffix_sum, memo);
                ret = ret.max(total - oppnent);
            }

            memo[i][m] = ret;
            ret
        }

        dfs(0, 1, &piles, &suffix_sum, &mut memo)
    }
}

fn main() {}
