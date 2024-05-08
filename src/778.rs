#![allow(dead_code)]

use std::collections::{BinaryHeap, HashSet};

struct Solution;

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let mut visited: HashSet<i32> = HashSet::new();
        let mut q = BinaryHeap::new();
        let n = grid.len() as i32;

        q.push((-grid[0][0], 0, 0));
        visited.insert(0);

        let mut ret = 0;
        let dirs = [[0, 1], [1, 0], [-1, 0], [0, -1]];
        while !q.is_empty() {
            let (height, x, y) = q.pop().unwrap();
            ret = ret.min(height);
            if x == n - 1 && y == n - 1 {
                return -ret;
            }
            for d in dirs.iter() {
                let nx = x + d[0];
                let ny = y + d[1];
                if nx < 0 || nx >= n || ny < 0 || ny >= n || visited.contains(&(nx * n + ny)) {
                    continue;
                }
                visited.insert(nx * n + ny);
                q.push((-grid[nx as usize][ny as usize], nx, ny));
            }
        }

        0
    }
}

fn main() {}
