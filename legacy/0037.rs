struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Self::solve(board);
    }
    fn solve(board: &mut Vec<Vec<char>>) -> bool {
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    for c in '1'..='9' {
                        if Self::is_valid_sudoku(board, i, j, c) {
                            board[i][j] = c;
                            if Self::solve(board) {
                                return true;
                            }
                            board[i][j] = '.';
                        }
                    }
                    return false;
                }
            }
        }
        true
    }

    fn is_valid_sudoku(board: &Vec<Vec<char>>, row: usize, col: usize, cur: char) -> bool {
        for i in 0..9 {
            if board[i][col] == cur {
                return false;
            }
            if board[row][i] == cur {
                return false;
            }
            let box_i = (row / 3) * 3 + i / 3;
            let box_j = (col / 3) * 3 + i % 3;
            if board[box_i][box_j] == cur {
                return false;
            }
        }
        true
    }
}
