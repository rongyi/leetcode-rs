#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn get_min_dist_sum(positions: Vec<Vec<i32>>) -> f64 {
        // Compute initial centroid as the average of all points
        let (mut x, mut y, n) = (0.0, 0.0, positions.len() as f64);

        for pos in &positions {
            x += pos[0] as f64 / n;
            y += pos[1] as f64 / n;
        }

        // Variables for gradient descent
        let mut step_size = 1.0;
        let min_step = 1e-6;

        while step_size > min_step {
            let mut best_x = x;
            let mut best_y = y;
            let mut min_dist = Self::compute_total_distance(&positions, x, y);
            let mut found_better = false;

            // Try moving in each direction
            for &(dx, dy) in &[
                (0.0, step_size),
                (0.0, -step_size),
                (step_size, 0.0),
                (-step_size, 0.0),
            ] {
                let new_x = x + dx;
                let new_y = y + dy;
                let dist = Self::compute_total_distance(&positions, new_x, new_y);

                if dist < min_dist {
                    best_x = new_x;
                    best_y = new_y;
                    min_dist = dist;
                    found_better = true;
                }
            }

            if found_better {
                x = best_x;
                y = best_y;
            } else {
                // If no improvement, reduce step size
                step_size /= 2.0;
            }
        }

        Self::compute_total_distance(&positions, x, y)
    }

    // Helper function to compute total Euclidean distance
    fn compute_total_distance(positions: &Vec<Vec<i32>>, x: f64, y: f64) -> f64 {
        let mut total_dist = 0.0;

        for pos in positions {
            let dx = x - pos[0] as f64;
            let dy = y - pos[1] as f64;
            total_dist += (dx * dx + dy * dy).sqrt();
        }

        total_dist
    }
}

fn main() {}
