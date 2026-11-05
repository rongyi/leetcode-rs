struct Solution;

use std::{collections::BinaryHeap, ops::Neg};
impl Solution {
    pub fn minimum_time(grid: Vec<Vec<i32>>) -> i32 {
        if grid[0][1] > 1 && grid[1][0] > 1 {
            return -1;
        }
        // weight, x, y, becuase we need to get lowest first, so here
        // weight put -weight in pq
        let mut pq: BinaryHeap<(i32, usize, usize)> = BinaryHeap::new();
        let m = grid.len();
        let n = grid[0].len();
        let mut visited = vec![vec![false; n]; m];
        pq.push((-grid[0][0], 0, 0));
        while let Some((mut cur_time, x, y)) = pq.pop() {
            cur_time = cur_time.abs();

            if x == m - 1 && y == n - 1 {
                return cur_time;
            }
            if visited[x][y] {
                continue;
            }
            visited[x][y] = true;
            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)].into_iter() {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                // let's say time for target cell is 4 and current time is 2, difference
                // = 2 (even).
                // Move to prev cell, time = 3
                // Move to curr cell, time = 4
                // Move to target cell, time = 5.
                // Hence we reach target cell with time: target cell time + 1 when
                // difference between target cell time and curr cell time is even.
                //
                // Let's say time for target cell is 5 and current time is 2, difference
                // = 3 (odd).
                // Move to prev cell, time = 3
                // Move to curr cell, time = 4
                // Move to target cell, time = 5.
                // Hence we reach target cell with time:
                // target cell time when difference between target cell time and curr
                // cell time is odd.
                let wait = if (grid[nx][ny] - cur_time) % 2 == 0 {
                    1
                } else {
                    0
                };
                pq.push(((grid[nx][ny] + wait).max(cur_time + 1).neg(), nx, ny));
            }
        }
        -1
    }
}

fn main() {}
