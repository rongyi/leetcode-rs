struct Solution;

impl Solution {
    pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut dp = vec![vec![0; 2001]; 1001];
        Self::dfs(&piles, k, 0, &mut dp)
    }

    fn dfs(piles: &Vec<Vec<i32>>, k: i32, i: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
        if i == piles.len() || k == 0 {
            return 0;
        }
        if dp[i][k as usize] != 0 {
            return dp[i][k as usize];
        }
        let mut sum = 0;
        for j in 0..=piles[i].len() as i32 {
            if k - j < 0 {
                break;
            }
            dp[i][k as usize] = dp[i][k as usize].max(sum + Self::dfs(piles, k - j, i + 1, dp));
            if j < piles[i].len() as i32 {
                sum += piles[i][j as usize];
            }
        }

        dp[i][k as usize]
    }
}

fn main() {}
