#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn hit_bricks(mut grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
        let m = grid.len();
        let n = grid[0].len();
        // hit them all and in reverse restore order
        for hit in hits.iter() {
            let (x, y) = (hit[0], hit[1]);
            if grid[x as usize][y as usize] == 0 {
                // mark as no op
                grid[x as usize][y as usize] = -1;
            } else {
                // real clear
                grid[x as usize][y as usize] = 0;
            }
        }
        for j in 0..n {
            Self::dfs(&mut grid, 0, j as i32);
        }
        let mut ret = vec![0; hits.len()];

        for (idx, hit) in hits.iter().enumerate().rev() {
            let (x, y) = (hit[0], hit[1]);

            if grid[x as usize][y as usize] == -1 {
                continue;
            }
            // restore
            grid[x as usize][y as usize] = 1;
            let mut need_recount = x == 0;
            // check neibor and find value 2
            if !need_recount {
                for d in [[0, 1], [1, 0], [-1, 0], [0, -1]].into_iter() {
                    let (nx, ny) = (x + d[0], y + d[1]);
                    if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 {
                        continue;
                    }
                    if grid[nx as usize][ny as usize] == 2 {
                        need_recount = true;
                        break;
                    }
                }
            }
            if need_recount {
                ret[idx] = Self::dfs(&mut grid, x, y) - 1;
            }
        }

        ret
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, x: i32, y: i32) -> i32 {
        if x < 0 || x >= grid.len() as i32 || y < 0 || y >= grid[0].len() as i32 {
            return 0;
        }
        if grid[x as usize][y as usize] != 1 {
            return 0;
        }
        // 2 means attach from ceil
        grid[x as usize][y as usize] = 2;

        let mut ret = 1;
        for d in [[0, 1], [1, 0], [0, -1], [-1, 0]].iter() {
            let (nx, ny) = (x + d[0], y + d[1]);
            ret += Self::dfs(grid, nx, ny);
        }

        return ret;
    }
}

fn main() {}
