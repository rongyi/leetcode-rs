struct Solution;

use std::i32;
impl Solution {
    pub fn shortest_bridge(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut first_done = false;
        let mut land1 = Vec::new();
        let mut land2 = Vec::new();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    if !first_done {
                        first_done = true;
                        Self::dfs(&mut grid, i as i32, j as i32, &mut land1);
                    } else {
                        Self::dfs(&mut grid, i as i32, j as i32, &mut land2);
                    }
                }
            }
        }
        let mut ret = i32::MAX;
        for &(x1, y1) in land1.iter() {
            for &(x2, y2) in land2.iter() {
                let cur = i32::abs(x1 - x2) + i32::abs(y1 - y2);
                ret = ret.min(cur);
            }
        }
        ret - 1
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, x: i32, y: i32, one_collect: &mut Vec<(i32, i32)>) {
        if x < 0 || x >= grid.len() as i32 || y < 0 || y >= grid[0].len() as i32 {
            return;
        }
        if grid[x as usize][y as usize] == 0 {
            return;
        }
        grid[x as usize][y as usize] = 0;
        one_collect.push((x, y));

        Self::dfs(grid, x + 1, y, one_collect);
        Self::dfs(grid, x - 1, y, one_collect);
        Self::dfs(grid, x, y + 1, one_collect);
        Self::dfs(grid, x, y - 1, one_collect);
    }
}

fn main() {}
