#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        if grid.is_empty() || grid[0].is_empty() {
            return false;
        }

        let rows = grid.len();
        let cols = grid[0].len();
        let mut visited = vec![vec![false; cols]; rows];

        for i in 0..rows {
            for j in 0..cols {
                if !visited[i][j] && Self::dfs(&grid, &mut visited, i, j, i, j, None) {
                    return true;
                }
            }
        }

        false
    }

    fn dfs(
        grid: &Vec<Vec<char>>,
        visited: &mut Vec<Vec<bool>>,
        i: usize,
        j: usize,
        prev_i: usize,
        prev_j: usize,
        prev_direction: Option<(i32, i32)>,
    ) -> bool {
        if visited[i][j] {
            return true;
        }

        visited[i][j] = true;

        let dirs: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let rows = grid.len() as i32;
        let cols = grid[0].len() as i32;
        let current_char = grid[i][j];

        for &(di, dj) in &dirs {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;

            // Skip if out of bounds
            if ni < 0 || ni >= rows || nj < 0 || nj >= cols {
                continue;
            }

            let ni = ni as usize;
            let nj = nj as usize;

            // Skip if this is the previous cell we came from
            if ni == prev_i && nj == prev_j {
                continue;
            }

            // Skip if the character doesn't match
            if grid[ni][nj] != current_char {
                continue;
            }

            // If we've visited this cell before, we found a cycle
            if visited[ni][nj] {
                return true;
            }

            if Self::dfs(grid, visited, ni, nj, i, j, Some((di, dj))) {
                return true;
            }
        }

        false
    }
}

fn main() {}
