struct Solution;

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let grid: Vec<Vec<char>> = grid.into_iter().map(|s| s.chars().collect()).collect();

        // Find start position and count total keys
        let mut start_r = 0;
        let mut start_c = 0;
        let mut total_keys = 0;

        for r in 0..rows {
            for c in 0..cols {
                match grid[r][c] {
                    '@' => {
                        start_r = r;
                        start_c = c;
                    }
                    'a'..='f' => total_keys += 1,
                    _ => {}
                }
            }
        }

        // BFS: (row, col, keys_mask) -> distance
        // Use a 3D visited array: [rows][cols][1 << total_keys]
        let max_mask = 1 << total_keys;
        let mut visited = vec![vec![vec![false; max_mask]; cols]; rows];

        let mut queue = VecDeque::new();
        queue.push_back((start_r, start_c, 0, 0)); // (r, c, keys_mask, dist)
        visited[start_r][start_c][0] = true;

        // Directions: up, down, left, right
        let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        while let Some((r, c, keys, dist)) = queue.pop_front() {
            // Check if we've collected all keys
            if keys == (1 << total_keys) - 1 {
                return dist;
            }

            for &(dr, dc) in &dirs {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;

                // Check bounds
                if nr < 0 || nr >= rows as i32 || nc < 0 || nc >= cols as i32 {
                    continue;
                }

                let nr = nr as usize;
                let nc = nc as usize;
                let cell = grid[nr][nc];

                // Skip walls
                if cell == '#' {
                    continue;
                }

                // Check if it's a lock and we don't have the key
                if cell.is_uppercase() {
                    let key_bit = cell.to_ascii_lowercase() as u8 - b'a';
                    if (keys & (1 << key_bit)) == 0 {
                        continue;
                    }
                }

                let mut new_keys = keys;

                // Collect key if present
                if cell.is_lowercase() {
                    let key_bit = cell as u8 - b'a';
                    new_keys |= 1 << key_bit;
                }

                // Visit this state if not visited
                if !visited[nr][nc][new_keys as usize] {
                    visited[nr][nc][new_keys as usize] = true;
                    queue.push_back((nr, nc, new_keys, dist + 1));
                }
            }
        }

        -1
    }
}

fn main() {}
