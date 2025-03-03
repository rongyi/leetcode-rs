#![allow(dead_code)]

struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        // Keep track of visited coordinates
        let mut visited = HashSet::new();

        // Start at origin (0, 0)
        let mut x = 0;
        let mut y = 0;

        // Add the starting position to visited
        visited.insert((x, y));

        // Process each direction
        for ch in path.chars() {
            // Update coordinates based on direction
            match ch {
                'N' => y += 1,
                'S' => y -= 1,
                'E' => x += 1,
                'W' => x -= 1,
                _ => {} // Ignore invalid directions
            }

            // Check if we've visited this coordinate before
            let current_pos = (x, y);
            if visited.contains(&current_pos) {
                return true; // Path crosses itself
            }

            // Add the new position to visited
            visited.insert(current_pos);
        }

        // If we never revisited a coordinate, the path doesn't cross itself
        false
    }
}

fn main() {}
