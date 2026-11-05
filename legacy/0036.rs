struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = vec![vec![false; 9]; 9];
        let mut cols = vec![vec![false; 9]; 9];
        let mut boxes = vec![vec![false; 9]; 9];

        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] != '.' {
                    let num = (board[i][j] as u8 - b'1') as usize;
                    let box_index = (i / 3) * 3 + j / 3;
                    if rows[i][num] || cols[j][num] || boxes[box_index][num] {
                        return false;
                    }

                    rows[i][num] = true;
                    cols[j][num] = true;
                    boxes[box_index][num] = true;
                }
            }
        }

        true
    }
}
