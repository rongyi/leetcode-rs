#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = is_water.len();
        let n = is_water[0].len();

        // Result matrix initialized with -1 (unprocessed)
        let mut heights = vec![vec![-1; n]; m];

        // Queue to do BFS, starting with all water cells (height 0)
        let mut queue = std::collections::VecDeque::new();

        // Initialize water cells with height 0 and add to queue
        for i in 0..m {
            for j in 0..n {
                if is_water[i][j] == 1 {
                    heights[i][j] = 0;
                    queue.push_back((i, j));
                }
            }
        }

        // Directions: up, right, down, left
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        // BFS to fill heights
        while let Some((i, j)) = queue.pop_front() {
            for &(di, dj) in &directions {
                let ni = i as i32 + di;
                let nj = j as i32 + dj;

                // Check if the new position is valid
                if ni >= 0 && ni < m as i32 && nj >= 0 && nj < n as i32 {
                    let ni = ni as usize;
                    let nj = nj as usize;

                    // Only process unvisited cells
                    if heights[ni][nj] == -1 {
                        heights[ni][nj] = heights[i][j] + 1;
                        queue.push_back((ni, nj));
                    }
                }
            }
        }

        heights
    }
}
fn main() {}
