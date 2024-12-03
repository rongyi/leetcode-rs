#![allow(dead_code)]
struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut visited = vec![vec![false; n]; n];
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    q.push_back((i, j));
                    visited[i][j] = true;
                }
            }
        }
        // no island or fullof island
        if q.is_empty() || q.len() == n * n {
            return -1;
        }

        let mut layer = -1;
        let dirs = [[0, 1], [1, 0], [0, -1], [-1, 0]];
        // take 1 as virus, and flood with bfs
        // each round means we get an 0 and we got a far away 0
        while !q.is_empty() {
            let sz = q.len();

            for _ in 0..sz {
                let (x, y) = q.pop_front().unwrap();
                for &[dx, dy] in dirs.iter() {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx < 0
                        || nx >= n as i32
                        || ny < 0
                        || ny >= n as i32
                        || visited[nx as usize][ny as usize]
                    {
                        continue;
                    }
                    q.push_back((nx as usize, ny as usize));
                    visited[nx as usize][ny as usize] = true;
                }
            }

            layer += 1;
        }

        layer
    }
}

fn main() {}
