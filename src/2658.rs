struct Solution;

impl Solution {
    pub fn find_max_fish(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut max_sum = 0;
        let m = grid.len();
        let n = grid[0].len();
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] > 0 {
                    let cur_sum = Self::dfs(&mut grid, i as i32, j as i32);
                    max_sum = max_sum.max(cur_sum);
                }
            }
        }
        max_sum
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, x: i32, y: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        if x < 0 || x >= m as i32 || y < 0 || y >= n as i32 {
            return 0;
        }
        // no way
        if grid[x as usize][y as usize] == 0 {
            return 0;
        }

        let mut sum = grid[x as usize][y as usize];
        // clean this route
        grid[x as usize][y as usize] = 0;
        sum += Self::dfs(grid, x + 1, y);
        sum += Self::dfs(grid, x - 1, y);
        sum += Self::dfs(grid, x, y + 1);
        sum += Self::dfs(grid, x, y - 1);

        sum
    }
}

fn main() {}
