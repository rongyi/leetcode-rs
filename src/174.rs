struct Solution;

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (dungeon.len(), dungeon[0].len());

        let mut dp = vec![vec![0; n]; m];

        dp[m - 1][n - 1] = (1 - dungeon[m - 1][n - 1]).max(1);

        for i in (0..m - 1).rev() {
            dp[i][n - 1] = (dp[i + 1][n - 1] - dungeon[i][n - 1]).max(1);
        }
        for j in (0..n - 1).rev() {
            dp[m - 1][j] = (dp[m - 1][j + 1] - dungeon[m - 1][j]).max(1);
        }
        for i in (0..m - 1).rev() {
            for j in (0..n - 1).rev() {
                let from_down = (dp[i + 1][j] - dungeon[i][j]).max(1);
                let from_right = (dp[i][j + 1] - dungeon[i][j]).max(1);
                dp[i][j] = from_down.min(from_right);
            }
        }

        dp[0][0]
    }
}

fn main() {}
