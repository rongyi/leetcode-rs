#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_days(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }

        let rows = grid.len();
        let cols = grid[0].len();

        // Check if the grid is already disconnected
        if Self::count_islands(&grid) != 1 {
            return 0;
        }

        // Try removing each land cell and check if it disconnects the grid
        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == 1 {
                    let mut new_grid = grid.clone();
                    new_grid[i][j] = 0;

                    if Self::count_islands(&new_grid) != 1 {
                        return 1;
                    }
                }
            }
        }

        // If removing one cell doesn't disconnect the grid, we need to remove 2 cells
        // There are two cases where 2 cells must be removed:
        // 1. When the grid forms a cycle, removing any single cell won't disconnect it
        // 2. When there are multiple paths between cells
        //
        // Since we've already checked all possible single cell removals and found
        // none that disconnect the grid, we know that the grid has at least one
        // redundant connection, requiring at least 2 cell removals to disconnect.

        // There's a theorem in graph theory that states that for a 2D grid
        // where each cell can only connect to adjacent cells (up, down, left, right),
        // the maximum number of cells that need to be removed to disconnect the grid
        // is 2, as long as the grid doesn't contain islands already.
        //
        // This is because:
        // - If there's a single critical point (articulation point), removing it disconnects the grid (1 day)
        // - If there are no articulation points, the grid forms some kind of loop structure
        // - In a 2D grid, any loop can be broken by removing at most 2 cells
        // - We can't have a case requiring 3 or more cells because in a 2D grid with only
        //   4-directional connectivity, there's no way to have 3 completely independent paths
        2
    }

    fn count_islands(grid: &Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut visited = vec![vec![false; cols]; rows];
        let mut count = 0;

        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == 1 && !visited[i][j] {
                    Self::dfs(grid, &mut visited, i, j, rows, cols);
                    count += 1;
                }
            }
        }

        count
    }

    fn dfs(
        grid: &Vec<Vec<i32>>,
        visited: &mut Vec<Vec<bool>>,
        i: usize,
        j: usize,
        rows: usize,
        cols: usize,
    ) {
        if i >= rows || j >= cols || grid[i][j] == 0 || visited[i][j] {
            return;
        }

        visited[i][j] = true;

        // Visit all four directions
        if i > 0 {
            Self::dfs(grid, visited, i - 1, j, rows, cols);
        }
        if j > 0 {
            Self::dfs(grid, visited, i, j - 1, rows, cols);
        }
        if i + 1 < rows {
            Self::dfs(grid, visited, i + 1, j, rows, cols);
        }
        if j + 1 < cols {
            Self::dfs(grid, visited, i, j + 1, rows, cols);
        }
    }
}

fn main() {}
