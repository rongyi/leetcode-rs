#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        let m = grid1.len();
        let n = grid1[0].len();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; n]; m];

        let mut acc = 0;

        for i in 0..m {
            for j in 0..n {
                if !visited[i][j] && grid2[i][j] == 1 {
                    let mut is_sub = true;
                    Self::dfs(
                        &grid1,
                        &grid2,
                        &mut visited,
                        i as i32,
                        j as i32,
                        &mut is_sub,
                    );
                    if is_sub {
                        acc += 1;
                    }
                }
            }
        }

        acc
    }
    fn dfs(
        grid1: &Vec<Vec<i32>>,
        grid2: &Vec<Vec<i32>>,
        visited: &mut Vec<Vec<bool>>,
        x: i32,
        y: i32,
        is_sub: &mut bool,
    ) {
        if x < 0 || x >= grid1.len() as i32 || y < 0 || y >= grid1[0].len() as i32 {
            return;
        }

        if visited[x as usize][y as usize] || grid2[x as usize][y as usize] == 0 {
            return;
        }

        visited[x as usize][y as usize] = true;

        if grid1[x as usize][y as usize] == 0 {
            *is_sub = false;
        }

        Self::dfs(grid1, grid2, visited, x + 1, y, is_sub);
        Self::dfs(grid1, grid2, visited, x - 1, y, is_sub);
        Self::dfs(grid1, grid2, visited, x, y + 1, is_sub);
        Self::dfs(grid1, grid2, visited, x, y - 1, is_sub);
    }
}

fn main() {}
