#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
        let mut xwin = false;
        let mut owin = false;
        let mut rows = vec![0; 3];
        let mut cols = vec![0; 3];
        let mut diag = 0;
        let mut anti_diag = 0;
        let mut turn = 0;
        let board: Vec<Vec<char>> = board.into_iter().map(|s| s.chars().collect()).collect();
        for i in 0..3 {
            for j in 0..3 {
                if board[i][j] == 'X' {
                    turn += 1;
                    if i == j {
                        diag += 1;
                    }
                    if i + j == 2 {
                        anti_diag += 1;
                    }
                    rows[i] += 1;
                    cols[j] += 1;
                } else if board[i][j] == 'O' {
                    turn -= 1;
                    if i == j {
                        diag -= 1;
                    }
                    if i + j == 2 {
                        anti_diag -= 1;
                    }
                    rows[i] -= 1;
                    cols[j] -= 1;
                }
            }
        }
        xwin = (diag == 3)
            || (anti_diag == 3)
            || rows.iter().any(|&num| num == 3)
            || cols.iter().any(|&num| num == 3);
        owin = (diag == -3)
            || (anti_diag == -3)
            || rows.iter().any(|&num| num == -3)
            || cols.iter().any(|&num| num == -3);
        if (xwin && turn == 0) || (owin && turn == 1) {
            return false;
        }

        (turn == 0 || turn == 1) && (!(xwin && owin))
    }
}

fn main() {
    let input = vec![1, 1, 1];
}
