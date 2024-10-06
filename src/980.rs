struct Solution;

impl Solution {
    pub fn unique_paths_iii(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut start_x = 0;
        let mut start_y = 0;
        let mut empty_cnt = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    start_x = i as i32;
                    start_y = j as i32;
                }

                if grid[i][j] == 0 {
                    empty_cnt += 1;
                }
            }
        }

        Self::dfs(&mut grid, start_x, start_y, empty_cnt, -1)
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, x: i32, y: i32, empty_cnt: i32, visited_empty: i32) -> i32 {
        if x < 0 || x >= grid.len() as i32 || y < 0 || y >= grid[0].len() as i32 {
            return 0;
        }
        if grid[x as usize][y as usize] == -1 {
            return 0;
        }
        if grid[x as usize][y as usize] == 2 {
            if visited_empty == empty_cnt {
                return 1;
            }
            return 0;
        }
        grid[x as usize][y as usize] = -1;
        let mut total_path = 0;

        let dirs = [[0, 1], [1, 0], [-1, 0], [0, -1]];
        for d in dirs.into_iter() {
            total_path += Self::dfs(grid, x + d[0], y + d[1], empty_cnt, visited_empty + 1);
        }
        grid[x as usize][y as usize] = 0;

        total_path
    }
}

fn main() {}
