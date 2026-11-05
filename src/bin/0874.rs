struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        // Directions: north, east, south, west
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut dir_idx = 0; // Start facing north
        let mut x = 0;
        let mut y = 0;
        let mut max_dist = 0;

        // Store obstacles in a HashSet for O(1) lookup
        let obstacle_set: HashSet<(i32, i32)> =
            obstacles.into_iter().map(|obs| (obs[0], obs[1])).collect();

        for &cmd in &commands {
            match cmd {
                -2 => {
                    // Turn left (counter-clockwise)
                    dir_idx = (dir_idx + 3) % 4;
                }
                -1 => {
                    // Turn right (clockwise)
                    dir_idx = (dir_idx + 1) % 4;
                }
                1..=9 => {
                    // Move forward
                    let (dx, dy) = directions[dir_idx];
                    for _ in 0..cmd {
                        let next_x = x + dx;
                        let next_y = y + dy;
                        if obstacle_set.contains(&(next_x, next_y)) {
                            break;
                        }
                        x = next_x;
                        y = next_y;
                        max_dist = max_dist.max(x * x + y * y);
                    }
                }
                _ => unreachable!(),
            }
        }

        max_dist
    }
}

fn main() {}
