struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let n = board.len();
        // 1. check row
        for i in 0..n {
            let mut visited = vec![-1; 10];
            for j in 0..n {
                if board[i][j] != '.' {
                    let idx = (board[i][j] as u8 - '0' as u8) as usize;
                    if visited[idx] != -1 {
                        return false;
                    }
                    visited[idx] = 0;
                }
            }
        }
        // 2. check column
        for j in 0..n {
            let mut visited = vec![-1; 10];
            for i in 0..n {
                if board[i][j] != '.' {
                    let idx = (board[i][j] as u8 - '0' as u8) as usize;
                    if visited[idx] != -1 {
                        return false;
                    }
                    visited[idx] = 0;
                }
            }
        }
        // 3. check 3x3 cell
        for r in (0..n).step_by(3) {
            for c in (0..n).step_by(3) {
                let mut visited = vec![-1; 10];
                for i in r..r + 3 {
                    for j in c..c + 3 {
                        if board[i][j] != '.' {
                            let idx = (board[i][j] as u8 - '0' as u8) as usize;
                            if visited[idx] != -1 {
                                return false;
                            }

                            visited[idx] = 0;
                        }
                    }
                }
            }
        }

        true
    }
}

fn main() {}
