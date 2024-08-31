struct Solution;

use std::collections::{HashSet, VecDeque};
impl Solution {
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        let grid: Vec<Vec<char>> = grid.into_iter().map(|r| r.chars().collect()).collect();
        let m = grid.len();
        let n = grid[0].len();
        let mut start = (0, 0);
        let mut total_keys = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '@' {
                    start = (i, j);
                } else if grid[i][j].is_ascii_lowercase() {
                    total_keys += 1;
                }
            }
        }
        let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        // (i, j, keys, step)
        let mut q = VecDeque::new();
        // (i, j, key state)
        let mut visited = HashSet::new();
        q.push_back((start.0, start.1, 0, 0));
        visited.insert((start.0, start.1, 0));

        while !q.is_empty() {
            let (x, y, keys, steps) = q.pop_front().unwrap();
            if keys == (1 << total_keys) - 1 {
                return steps;
            }
            for &(dx, dy) in dirs.iter() {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                // out of map
                if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                // wall
                let val = grid[nx][ny];
                if val == '#' {
                    continue;
                }
                // clone
                let mut next_keys = keys;
                if val.is_ascii_lowercase() {
                    next_keys |= 1 << (val as u8 - b'a');
                }
                if val.is_ascii_uppercase() && (keys & (1 << (val as u8 - b'A'))) == 0 {
                    continue;
                }
                if !visited.contains(&(nx, ny, next_keys)) {
                    visited.insert((nx, ny, next_keys));
                    q.push_back((nx, ny, next_keys, steps + 1));
                }
            }
        }

        -1
    }
}

fn main() {}
