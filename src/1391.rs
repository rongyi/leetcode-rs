#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        let mut visited = vec![vec![false; n]; m];

        let directions = vec![
            vec![],
            vec![(0, -1), (0, 1)],  // 1: left and right
            vec![(-1, 0), (1, 0)],  // 2: up and down
            vec![(0, -1), (1, 0)],  // 3: left and down
            vec![(0, 1), (1, 0)],   // 4: right and down
            vec![(-1, 0), (0, -1)], // 5: up and left
            vec![(-1, 0), (0, 1)],  // 6: up and right
        ];
        Self::dfs(0, 0, m, n, &grid, &mut visited, &directions)
    }

    fn dfs(
        x: usize,
        y: usize,
        m: usize,
        n: usize,
        grid: &Vec<Vec<i32>>,
        visited: &mut Vec<Vec<bool>>,
        directions: &Vec<Vec<(i32, i32)>>,
    ) -> bool {
        if x == m - 1 && y == n - 1 {
            return true;
        }
        visited[x][y] = true;
        let cur_street = grid[x][y] as usize;

        for &(dx, dy) in directions[cur_street].iter() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0
                || nx >= m as i32
                || ny < 0
                || ny >= n as i32
                || visited[nx as usize][ny as usize]
            {
                continue;
            }

            let next_street = grid[nx as usize][ny as usize] as usize;
            if Self::are_street_connected(next_street, dx, dy, directions) {
                if Self::dfs(nx as usize, ny as usize, m, n, grid, visited, directions) {
                    return true;
                }
            }
        }

        false
    }

    fn are_street_connected(
        next_street: usize,
        dx: i32,
        dy: i32,
        directions: &Vec<Vec<(i32, i32)>>,
    ) -> bool {
        for &(dx1, dy1) in directions[next_street].iter() {
            if dx == -dx1 && dy == -dy1 {
                return true;
            }
        }
        false
    }
}

fn main() {}
