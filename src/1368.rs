#![allow(dead_code)]

struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        // right, left, down, up, using index to compare
        let m = grid.len();
        let n = grid[0].len();

        let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

        let mut q = VecDeque::new();
        q.push_back((0, 0));

        let mut dist = vec![vec![i32::MAX; n]; m];
        dist[0][0] = 0;

        while let Some((x, y)) = q.pop_front() {
            let cur_cost = dist[x][y];
            if x == m - 1 && y == n - 1 {
                return cur_cost;
            }

            for (idx, &(dx, dy)) in dirs.iter().enumerate() {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                // invalid?
                if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 {
                    continue;
                }

                let nx = nx as usize;
                let ny = ny as usize;
                // the index is precisely ordered
                // check the from address
                let additional_cost = if grid[x][y] as usize == idx + 1 { 0 } else { 1 };

                let new_cost = cur_cost + additional_cost;

                if new_cost < dist[nx][ny] {
                    dist[nx][ny] = new_cost;
                    if additional_cost == 0 {
                        // this is the keypoint, mininum cost get insert to front
                        q.push_front((nx, ny));
                    } else {
                        q.push_back((nx, ny));
                    }
                }
            }
        }

        dist[m - 1][n - 1]
    }
}

fn main() {}
