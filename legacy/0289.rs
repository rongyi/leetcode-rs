struct Solution;

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let m = board.len();
        let n = board[0].len();
        let mut shadow = board.clone();
        let directions = [
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];
        for i in 0..m {
            for j in 0..n {
                let live = Self::live_neighbors(board, i as i32, j as i32, m, n, &directions);
                if board[i][j] == 1 {
                    if live < 2 || live > 3 {
                        shadow[i][j] = 0;
                    }
                } else if board[i][j] == 0 {
                    if live == 3 {
                        shadow[i][j] = 1;
                    }
                }
            }
        }

        *board = shadow;
    }

    fn live_neighbors(
        board: &Vec<Vec<i32>>,
        x: i32,
        y: i32,
        m: usize,
        n: usize,
        dirs: &[(i32, i32)],
    ) -> i32 {
        let mut ret = 0;
        for &(dx, dy) in dirs {
            let nx = x + dx;
            let ny = y + dy;

            if nx < 0 || nx as usize >= m || ny < 0 || ny as usize >= n {
                continue;
            }

            if board[nx as usize][ny as usize] == 1 {
                ret += 1;
            }
        }

        ret
    }
}

fn main() {
    unimplemented!();
}
