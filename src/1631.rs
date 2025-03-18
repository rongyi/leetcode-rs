#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let rows = heights.len();
        let cols = heights[0].len();

        // Initialize distance matrix with infinity
        let mut efforts = vec![vec![i32::MAX; cols]; rows];
        efforts[0][0] = 0;

        // Define directions: up, right, down, left
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        // Min-heap for Dijkstra's algorithm (effort, row, col)
        let mut min_heap = BinaryHeap::new();
        min_heap.push(Reverse((0, 0, 0))); // Start from (0,0) with 0 effort

        while let Some(Reverse((effort, r, c))) = min_heap.pop() {
            // If we've reached the destination, return the effort
            if r == rows - 1 && c == cols - 1 {
                return effort;
            }

            // If we've found a better path already, skip
            if effort > efforts[r][c] {
                continue;
            }

            // Check all four directions
            for &(dr, dc) in &directions {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;

                // Check if the new position is valid
                if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                    let nr = nr as usize;
                    let nc = nc as usize;

                    // Calculate new effort (max of current effort and height difference)
                    let new_effort = effort.max((heights[nr][nc] - heights[r][c]).abs());

                    // If we found a better path, update and add to heap
                    if new_effort < efforts[nr][nc] {
                        efforts[nr][nc] = new_effort;
                        min_heap.push(Reverse((new_effort, nr, nc)));
                    }
                }
            }
        }

        efforts[rows - 1][cols - 1] // Return effort to reach destination
    }
}

fn main() {}
