struct Solution;

impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        let mut cnt = 0;
        let m = board.len();
        let n = board[0].len();
        for i in 0..m {
            for j in 0..n {
                if board[i][j] == 'X' &&
                (i == 0 || board[i - 1][j] == '.') &&
                (j == 0 || board[i][j - 1] == '.') {
                    cnt += 1;
                }
            }
        }



        cnt
    }
}

fn main() {}
