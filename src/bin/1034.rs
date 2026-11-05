struct Solution;

impl Solution {
    pub fn color_border(mut grid: Vec<Vec<i32>>, row: i32, col: i32, color: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let original_color = grid[row as usize][col as usize];
        let mut visited = vec![vec![false; n]; m];

        fn dfs(
            grid: &mut Vec<Vec<i32>>,
            visited: &mut Vec<Vec<bool>>,
            x: i32,
            y: i32,
            original_color: i32,
            color: i32,
        ) {
            let m = grid.len() as i32;
            let n = grid[0].len() as i32;

            if x < 0
                || x >= m
                || y < 0
                || y >= n
                || visited[x as usize][y as usize]
                || grid[x as usize][y as usize] != original_color
            {
                return;
            }

            visited[x as usize][y as usize] = true;
            let is_border = x == 0
                || x == m - 1
                || y == 0
                || y == n - 1
                || grid[x as usize][(y + 1) as usize] != original_color
                || grid[x as usize][(y - 1) as usize] != original_color
                || grid[(x + 1) as usize][y as usize] != original_color
                || grid[(x - 1) as usize][y as usize] != original_color;

            dfs(grid, visited, x + 1, y, original_color, color);
            dfs(grid, visited, x - 1, y, original_color, color);
            dfs(grid, visited, x, y + 1, original_color, color);
            dfs(grid, visited, x, y - 1, original_color, color);

            if is_border {
                grid[x as usize][y as usize] = color;
            }
        }

        dfs(&mut grid, &mut visited, row, col, original_color, color);

        grid
    }
}

fn main() {}
