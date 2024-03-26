struct Solution;

impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let max_move = max_move as usize;
        let start_row = start_row as usize;
        let start_column = start_column as usize;

        let mut dp = vec![vec![vec![0i64; n]; m]; max_move + 1];
        for step in 1..=max_move {
            for i in 0..m {
                for j in 0..n {
                    let val1 = if i == 0 { 1 } else { dp[step - 1][i - 1][j] };
                    let val2 = if i == m - 1 {
                        1
                    } else {
                        dp[step - 1][i + 1][j]
                    };
                    let val3 = if j == 0 { 1 } else { dp[step - 1][i][j - 1] };
                    let val4 = if j == n - 1 {
                        1
                    } else {
                        dp[step - 1][i][j + 1]
                    };
                    dp[step][i][j] = (val1 + val2 + val3 + val4) % 1000000007;
                }
            }
        }

        dp[max_move][start_row as usize][start_column as usize] as i32
    }
}

fn main() {}
