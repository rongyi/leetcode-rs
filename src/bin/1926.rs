#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        use std::collections::VecDeque;

        let rows = maze.len() as i32;
        let cols = maze[0].len() as i32;
        let start_row = entrance[0];
        let start_col = entrance[1];

        let mut queue = VecDeque::new();
        let mut visited = vec![vec![false; cols as usize]; rows as usize];

        // Add entrance to queue and mark as visited
        queue.push_back((start_row, start_col, 0)); // (row, col, steps)
        visited[start_row as usize][start_col as usize] = true;

        // Directions: up, right, down, left
        let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

        while let Some((row, col, steps)) = queue.pop_front() {
            // Check if we're at a border and not at the entrance
            if (row == 0 || row == rows - 1 || col == 0 || col == cols - 1)
                && (row != start_row || col != start_col)
            {
                return steps;
            }

            // Try all four directions
            for (dr, dc) in &directions {
                let new_row = row + dr;
                let new_col = col + dc;

                // Check if the new position is valid
                if new_row >= 0
                    && new_row < rows
                    && new_col >= 0
                    && new_col < cols
                    && !visited[new_row as usize][new_col as usize]
                    && maze[new_row as usize][new_col as usize] == '.'
                {
                    visited[new_row as usize][new_col as usize] = true;
                    queue.push_back((new_row, new_col, steps + 1));
                }
            }
        }

        -1 // No exit found
    }
}

fn main() {}
