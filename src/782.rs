#![allow(dead_code)]

struct Solution;

impl Solution {
    // 其实不懂，照搬c++
    pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();

        for i in 0..n {
            for j in 0..n {
                if board[0][0] ^ board[0][j] ^ board[i][j] ^ board[i][0] == 1 {
                    return -1;
                }
            }
        }
        let mut row_sum = 0;
        let mut col_sum = 0;
        for i in 0..n {
            row_sum += board[0][i];
            col_sum += board[i][0];
        }
        let n = n as i32;
        if (n >> 1) > row_sum
            || row_sum > ((n + 1) >> 1)
            || (n >> 1) > col_sum
            || col_sum > ((n + 1) >> 1)
        {
            return -1;
        }
        let mut row_swap = 0;
        let mut col_swap = 0;

        for i in 0..n {
            if (i & 1) != board[i as usize][0] {
                row_swap += 1;
            }
            if (i & 1) != board[0][i as usize] {
                col_swap += 1;
            }
        }
        if (n & 1) == 1 {
            if row_swap & 1 == 1 {
                row_swap = n as i32 - row_swap;
            }
            if col_swap & 1 == 1 {
                col_swap = n as i32 - col_swap;
            }
        } else {
            row_swap = row_swap.min(n as i32 - row_swap);
            col_swap = col_swap.min(n as i32 - col_swap);
        }

        (row_swap + col_swap) >> 1
    }
}

fn main() {}
