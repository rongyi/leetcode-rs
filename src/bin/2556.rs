struct Solution;

impl Solution {
    pub fn is_possible_to_cut_path(mut grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        let first_round = Self::dfs(&mut grid, m, n, 0, 0);
        if !first_round {
            return true;
        }

        grid[0][0] = 1;

        !Self::dfs(&mut grid, m, n, 0, 0)
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, m: usize, n: usize, x: usize, y: usize) -> bool {
        if x >= m || y >= n || grid[x][y] == 0 {
            return false;
        }
        if x == m - 1 && y == n - 1 {
            return true;
        }
        grid[x][y] = 0;

        Self::dfs(grid, m, n, x + 1, y) || Self::dfs(grid, m, n, x, y + 1)
    }
}

fn main() {}
