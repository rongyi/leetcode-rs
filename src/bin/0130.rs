struct Solution;

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let (m, n) = (board.len(), board[0].len());
        for i in 0..m {
            Self::set_o(board, i as i32, 0);
            Self::set_o(board, i as i32, (n - 1) as i32);
        }
        for j in 0..n {
            Self::set_o(board, 0, j as i32);
            Self::set_o(board, m as i32 - 1, j as i32);
        }

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
    fn set_o(board: &mut Vec<Vec<char>>, x: i32, y: i32) {
        let m = board.len() as i32;
        let n = board[0].len() as i32;

        if x < 0 || x >= m || y < 0 || y >= n {
            return;
        }
        if board[x as usize][y as usize] == 'O' {
            // mark it safe
            board[x as usize][y as usize] = 'S';
            Self::set_o(board, x + 1, y);
            Self::set_o(board, x - 1, y);
            Self::set_o(board, x, y + 1);
            Self::set_o(board, x, y - 1);
        }
    }
}

fn main() {}
