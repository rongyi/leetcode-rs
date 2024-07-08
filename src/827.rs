#![allow(dead_code)]

use std::{collections::{HashMap, HashSet}};

struct Solution;

impl Solution {
    pub fn largest_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let dirs = vec![[0, 1], [1, 0], [0, -1], [-1, 0]];

        let mut ret = 0;

        // 0, 1 is original value, and we don't have visited flag, so
        // we change 1 to this index value
        let mut index = 2;
        let mut area: HashMap<i32, i32> = HashMap::new();
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    let cur_area = Self::dfs(&mut grid, &dirs, i as i32, j as i32, index);
                    ret = ret.max(cur_area);
                    area.insert(index, cur_area);
                    
                    index += 1;
                }
            }
        }
        
        // tranverse all 0 node
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] != 0 {
                    continue;
                }
                let mut visited: HashSet<i32> = HashSet::new();
                let mut cur = 1;
                
                for d in dirs.iter() {
                    let nx = i as i32 + d[0];
                    let ny = j as i32 + d[1];
                    if nx < 0 || nx >= n as i32 || ny < 0 || ny >= n as i32 {
                        continue;
                    }
                    let cur_idx = grid[nx as usize][ny as usize];
                    if cur_idx > 1 && !visited.contains(&cur_idx) {
                        visited.insert(cur_idx);
                        cur += area[&cur_idx];
                    }
                }
                ret = ret.max(cur);
            }
        }

        ret
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, dirs: &Vec<[i32; 2]>, x: i32, y: i32, cur_index: i32) -> i32 {
        let mut area = 1;
        grid[x as usize][y as usize] = cur_index;

        for d in dirs.iter() {
            let nx = x + d[0];
            let ny = y + d[1];
            if nx < 0 || nx >= grid.len() as i32 || ny < 0 || ny >= grid.len() as i32 {
                continue;
            }
            if grid[nx as usize][ny as usize] == 1 {
                area += Self::dfs(grid, dirs, nx, ny, cur_index);
            }
        }

        area
    }
}

fn main() {}
