struct Solution;

use std::{collections::VecDeque, i32};
impl Solution {
    pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();
        q.push_back((0, 0));
        let mut dp = vec![vec![i32::MAX; n]; m];
        dp[0][0] = 0;
        let mut visited = vec![vec![false; n]; m];
        visited[0][0] = true;

        while let Some((x, y)) = q.pop_front() {
            for d in [[0, 1], [1, 0], [0, -1], [-1, 0]].iter() {
                let nx = x as i32 + d[0];
                let ny: i32 = y as i32 + d[1];
                if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                if visited[nx][ny] {
                    continue;
                }
                dp[nx][ny] = dp[x][y] + if grid[nx][ny] == 1 { 1 } else { 0 };
                if grid[nx][ny] == 1 {
                    q.push_back((nx, ny));
                } else {
                    q.push_front((nx, ny));
                }
                visited[nx][ny] = true;
            }
        }

        dp[m - 1][n - 1]
    }
}

fn main() {}
