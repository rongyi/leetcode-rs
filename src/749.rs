#![allow(dead_code)]
struct Solution;

use std::collections::{BinaryHeap, HashSet};

#[derive(Debug, PartialEq, Eq)]
struct Cluster {
    contaminated: HashSet<(usize, usize)>,
    uncontaminated: HashSet<(usize, usize)>,
    wall_cnt: i32,
}

impl Cluster {
    fn new() -> Self {
        Self {
            contaminated: HashSet::new(),
            uncontaminated: HashSet::new(),
            wall_cnt: 0,
        }
    }
}

impl Ord for Cluster {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.uncontaminated.len().cmp(&other.uncontaminated.len())
    }
}

impl PartialOrd for Cluster {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn contain_virus(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut ret = 0;

        loop {
            let mut visited = vec![vec![false; n]; m];
            let mut pq = BinaryHeap::new();

            for i in 0..m {
                for j in 0..n {
                    if !visited[i][j] && grid[i][j] == 1 {
                        let mut c = Cluster::new();
                        Self::dfs(&grid, &mut c, i as i32, j as i32, &mut visited);
                        pq.push(c);
                    }
                }
            }
            if pq.is_empty() {
                break;
            }

            let cur_cluster = pq.pop().unwrap();

            for &(x, y) in cur_cluster.contaminated.iter() {
                grid[x][y] = -1;
            }
            ret += cur_cluster.wall_cnt;

            while !pq.is_empty() {
                let cur_cluster = pq.pop().unwrap();

                for &(x, y) in cur_cluster.uncontaminated.iter() {
                    grid[x][y] = 1;
                }
            }
        }

        ret
    }

    // cluster is like current snapshot
    fn dfs(grid: &Vec<Vec<i32>>, c: &mut Cluster, i: i32, j: i32, visited: &mut Vec<Vec<bool>>) {
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;
        if i < 0
            || i >= m
            || j < 0
            || j >= n
            || visited[i as usize][j as usize]
            || grid[i as usize][j as usize] == -1
        {
            return;
        }

        if grid[i as usize][j as usize] == 0 {
            c.wall_cnt += 1;
            c.uncontaminated.insert((i as usize, j as usize));
            return;
        }

        // a virus in (i, j)
        c.contaminated.insert((i as usize, j as usize));
        visited[i as usize][j as usize] = true;

        Self::dfs(grid, c, i + 1, j, visited);
        Self::dfs(grid, c, i - 1, j, visited);
        Self::dfs(grid, c, i, j + 1, visited);
        Self::dfs(grid, c, i, j - 1, visited);
    }
}

fn main() {}
