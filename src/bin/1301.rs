#![allow(dead_code)]

struct Solution;

impl Solution {
    const MOD: i64 = 1e9 as i64 + 7;
    pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
        let mut board: Vec<Vec<char>> = board.into_iter().map(|s| s.chars().collect()).collect();
        let sz = board.len();
        let mut score = vec![vec![0; sz + 1]; sz + 1];
        let mut path = vec![vec![0; sz + 1]; sz + 1];

        board[0][0] = '0';
        board[sz - 1][sz - 1] = '0';
        path[0][0] = 1;

        for i in 1..=sz {
            for j in 1..=sz {
                if board[i - 1][j - 1] == 'X' {
                    continue;
                }
                // in one move you can go up, left or up-left (diagonally) only if there is no obstacle there.
                for &(dx, dy) in [(0, 1), (1, 0), (1, 1)].iter() {
                    let nx = i - dx;
                    let ny = j - dy;
                    let val = score[nx][ny] + (board[i - 1][j - 1] as u8 - '0' as u8) as i64;

                    if score[i][j] <= val {
                        path[i][j] = (if score[i][j] == val { path[i][j] } else { 0 }
                            + path[nx][ny])
                            % Self::MOD;
                        score[i][j] = val;
                    }
                }
            }
        }

        if path[sz][sz] > 0 {
            vec![score[sz][sz] as i32, path[sz][sz] as i32]
        } else {
            vec![0, 0]
        }
    }
}

fn main() {}
