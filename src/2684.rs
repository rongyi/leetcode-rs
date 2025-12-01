struct Solution;

impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut ret = 0;
        let mut cache: Vec<Vec<i32>> = vec![vec![-1; n]; m];

        for i in 0..m {
            let cur = Self::dfs(&grid, i, 0, 0 /* step */, &mut cache);
            ret = ret.max(cur);
        }

        ret
    }
    fn dfs(grid: &Vec<Vec<i32>>, x: usize, y: usize, step: i32, cache: &mut Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        if y + 1 == n {
            cache[x][y] = step;
            return step;
        }
        if cache[x][y] != -1 {
            return cache[x][y];
        }
        let ny = y + 1;
        let val = grid[x][y];
        let mut ret = step;
        if x > 0 && grid[x - 1][ny] > val {
            let v1 = Self::dfs(grid, x - 1, ny, step + 1, cache);
            ret = ret.max(v1);
        }
        if grid[x][ny] > val {
            let v2 = Self::dfs(grid, x, ny, step + 1, cache);
            ret = ret.max(v2);
        }
        if x + 1 < m && grid[x + 1][ny] > val {
            let v3 = Self::dfs(grid, x + 1, ny, step + 1, cache);
            ret = ret.max(v3);
        }

        cache[x][y] = ret;
        ret
    }
}

fn main() {}
