#![allow(dead_code)]
struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        if k >= (m + n - 2) as i32 {
            return (m + n - 2) as i32;
        }
        let mut q = VecDeque::new();
        // x, y, k, path_acc
        q.push_back((0, 0, k, 0));

        let mut visited = vec![vec![vec![false; k as usize + 1]; n]; m];
        visited[0][0][k as usize] = true;
        let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        while let Some((x, y, remain_k, steps)) = q.pop_front() {
            if x == m - 1 && y == n - 1 {
                return steps;
            }

            for &(dx, dy) in dirs.iter() {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;

                let new_k = remain_k - grid[nx][ny];
                if new_k >= 0 && !visited[nx][ny][new_k as usize] {
                    visited[nx][ny][new_k as usize] = true;
                    q.push_back((nx, ny, new_k, steps + 1));
                }
            }
        }

        -1
    }
}

fn main() {}
