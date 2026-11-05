struct Solution;

impl Solution {
    pub fn count_paths(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut memo: Vec<Vec<i32>> = vec![vec![0; n]; m];

        let mut acc = 0;

        for i in 0..m {
            for j in 0..n {
                acc = (acc + Self::dfs(&grid, &mut memo, i, j)) % 1_000_000_007;
            }
        }

        acc
    }
    fn dfs(grid: &Vec<Vec<i32>>, memo: &mut Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
        if memo[x][y] != 0 {
            return memo[x][y];
        }
        let mut acc = 1;

        for &(dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || nx >= grid.len() as i32 || ny < 0 || ny >= grid[0].len() as i32 {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if grid[x][y] < grid[nx][ny] {
                acc = (acc + Self::dfs(grid, memo, nx, ny)) % 1_000_000_007;
            }
        }
        memo[x][y] = acc;

        acc
    }
}

fn main() {}
