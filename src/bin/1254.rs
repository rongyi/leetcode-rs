#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        // 沿着边把iland全去掉，因为完全包围不包括边
        for i in 0..m {
            Self::fill(&mut grid, i as i32, 0);
            Self::fill(&mut grid, i as i32, n as i32 - 1);
        }

        for j in 0..n {
            Self::fill(&mut grid, 0, j as i32);
            Self::fill(&mut grid, m as i32 - 1, j as i32);
        }

        let mut acc = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 {
                    acc += 1;
                    Self::fill(&mut grid, i as i32, j as i32);
                }
            }
        }
        acc
    }
    //
    fn fill(grid: &mut Vec<Vec<i32>>, x: i32, y: i32) {
        if x < 0
            || x >= grid.len() as i32
            || y < 0
            || y >= grid[0].len() as i32
            || grid[x as usize][y as usize] == 1
        {
            return;
        }

        grid[x as usize][y as usize] = 1;
        Self::fill(grid, x + 1, y);
        Self::fill(grid, x - 1, y);
        Self::fill(grid, x, y + 1);
        Self::fill(grid, x, y - 1);
    }
}

fn main() {}
