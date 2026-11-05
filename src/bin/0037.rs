struct Solution;

impl Solution {
    pub fn solve_sudoku(grid: &mut Vec<Vec<char>>) {
        Self::try_solve(grid, 0, 0);
    }
    fn try_solve(grid: &mut Vec<Vec<char>>, i: usize, j: usize) -> bool {
        if j == 9 {
            return Self::try_solve(grid, i + 1, 0);
        }
        if i == 9 {
            return true;
        }
        if grid[i][j] != '.' {
            return Self::try_solve(grid, i, j + 1);
        }

        for c in '1'..='9' {
            if Self::is_valid_placement(grid, i, j, c) {
                grid[i][j] = c;
                if Self::try_solve(grid, i, j + 1) {
                    return true;
                }
                grid[i][j] = '.';
            }
        }

        false
    }

    fn is_valid_placement(board: &Vec<Vec<char>>, row: usize, col: usize, num: char) -> bool {
        // Check row
        for j in 0..9 {
            if board[row][j] == num {
                return false;
            }
        }
        // Check column
        for i in 0..9 {
            if board[i][col] == num {
                return false;
            }
        }
        // Check 3x3 box
        let box_row = row / 3 * 3;
        let box_col = col / 3 * 3;
        for i in box_row..box_row + 3 {
            for j in box_col..box_col + 3 {
                if board[i][j] == num {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {}
