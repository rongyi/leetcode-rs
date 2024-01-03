struct Solution;

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let m = board.len() as i32;
        let n = board[0].len() as i32;

        for i in 0..m {
            Self::dfs(board, i, 0);
            Self::dfs(board, i, n - 1);
        }
        for j in 0..n {
            Self::dfs(board, 0, j);
            Self::dfs(board, m - 1, j);
        }

        let m = board.len();
        let n = board[0].len();

        for i in 0..m {
            for j in 0..n {
                if board[i][j] == 'O' {
                    board[i][j] = 'X';
                } else if board[i][j] == 'S' {
                    board[i][j] = 'O';
                }
            }
        }
    }
    fn dfs(board: &mut Vec<Vec<char>>, i: i32, j: i32) {
        let m = board.len() as i32;
        let n = board[0].len() as i32;
        if i < 0 || i >= m || j < 0 || j >= n || board[i as usize][j as usize] != 'O' {
            return;
        }
        board[i as usize][j as usize] = 'S';

        Self::dfs(board, i + 1, j);
        Self::dfs(board, i - 1, j);
        Self::dfs(board, i, j + 1);
        Self::dfs(board, i, j - 1);
    }
}

fn main() {}
