
struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn maximum_minutes(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let dirs = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

        let bfs = |q: &mut VecDeque<(usize, usize)>| {
            let mut wald_grid = vec![vec![0; n]; m];
            while let Some((x, y)) = q.pop_front() {
                for &(dx, dy) in dirs.iter() {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 {
                        continue;
                    }
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if grid[nx][ny] == 0 && wald_grid[nx][ny] == 0 {
                        wald_grid[nx][ny] = wald_grid[x][y] + 1;
                        q.push_back((nx, ny));
                    }
                }
            }

            (
                wald_grid[m - 1][n - 1],
                wald_grid[m - 2][n - 1],
                wald_grid[m - 1][n - 2],
            )
        };

        let mut fires: VecDeque<(usize, usize)> = VecDeque::new();
        let mut person = VecDeque::new();
        person.push_back((0, 0));
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    fires.push_back((i, j));
                }
            }
        }
        let fire_steps = bfs(&mut fires);
        let person_steps = bfs(&mut person);
        if fire_steps.0 == 0 && person_steps.0 != 0 {
            return 1e9 as _;
        }
        let mut diff = fire_steps.0 - person_steps.0;
        if person_steps.0 != 0 && diff >= 0 {
            if fire_steps.1 - person_steps.1 <= diff && fire_steps.2 - person_steps.2 <= diff {
                diff -= 1;
            }

            return diff;
        }

        -1
    }
}

fn main() {}
