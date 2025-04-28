#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        let rows = grid2.len();
        let cols = grid2[0].len();
        let mut visited = vec![vec![false; cols]; rows];
        let mut count = 0;

        // Helper function to perform DFS
        fn dfs(
            grid1: &[Vec<i32>],
            grid2: &[Vec<i32>],
            visited: &mut [Vec<bool>],
            r: usize,
            c: usize,
            is_subisland: &mut bool,
        ) {
            let rows = grid2.len();
            let cols = grid2[0].len();

            // Check if out of bounds or already visited or not land in grid2
            if r >= rows || c >= cols || visited[r][c] || grid2[r][c] == 0 {
                return;
            }

            // Mark as visited
            visited[r][c] = true;

            // If the corresponding cell in grid1 is not land, this is not a sub-island
            if grid1[r][c] == 0 {
                *is_subisland = false;
            }

            // Explore all four directions
            let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
            for (dr, dc) in directions {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;

                if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                    dfs(
                        grid1,
                        grid2,
                        visited,
                        nr as usize,
                        nc as usize,
                        is_subisland,
                    );
                }
            }
        }

        // Iterate through each cell in grid2
        for r in 0..rows {
            for c in 0..cols {
                // If it's an unvisited land in grid2
                if grid2[r][c] == 1 && !visited[r][c] {
                    let mut is_subisland = true;
                    dfs(&grid1, &grid2, &mut visited, r, c, &mut is_subisland);

                    if is_subisland {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

fn main() {}
