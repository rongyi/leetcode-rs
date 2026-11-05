struct Solution;
impl Solution {
    pub fn update_board(mut board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
        let m = board.len();
        let n = board[0].len();
        let dirs = [
            [0, 1],
            [1, 0],
            [0, -1],
            [-1, 0],
            [-1, -1],
            [-1, 1],
            [1, 1],
            [1, -1],
        ];
        let mut mines = vec![vec![0; n]; m];
        Self::count_mines(&board, &mut mines, m as i32, n as i32, &dirs);
        let x = click[0];
        let y = click[1];
        if board[x as usize][y as usize] == 'M' {
            let mut ret = board.clone();
            ret[x as usize][y as usize] = 'X';
            return ret;
        }
        let mut visited = vec![vec![false; n]; m];

        Self::dfs(
            &mut board,
            &mines,
            x,
            y,
            m as i32,
            n as i32,
            &mut visited,
            &dirs,
        );

        board
    }

    fn dfs(
        board: &mut Vec<Vec<char>>,
        count: &Vec<Vec<i32>>,
        x: i32,
        y: i32,
        m: i32,
        n: i32,
        visited: &mut Vec<Vec<bool>>,
        dirs: &[[i32; 2]],
    ) {
        if x < 0 || x >= m || y < 0 || y >= n || visited[x as usize][y as usize] {
            return;
        }
        let ux = x as usize;
        let uy = y as usize;
        visited[ux][uy] = true;
        // recursive
        if count[ux][uy] == 0 {
            // blank
            board[ux][uy] = 'B';
            for d in dirs.iter() {
                let nx = x + d[0];
                let ny = y + d[1];
                Self::dfs(board, count, nx, ny, m, n, visited, dirs);
            }
        } else {
            // just indicate the number surround
            board[ux][uy] = ('0' as u8 + count[ux][uy] as u8) as char;
        }
    }

    fn count_mines(
        board: &Vec<Vec<char>>,
        mines: &mut Vec<Vec<i32>>,
        m: i32,
        n: i32,
        dirs: &[[i32; 2]],
    ) {
        for i in 0..m {
            for j in 0..n {
                // a bomb
                if board[i as usize][j as usize] == 'M' {
                    // update neibs
                    for d in dirs.iter() {
                        let nx = i + d[0];
                        let ny = j + d[1];
                        if nx < 0 || nx >= m || ny < 0 || ny >= n {
                            continue;
                        }
                        mines[nx as usize][ny as usize] += 1;
                    }
                }
            }
        }
    }
}

fn main() {}
