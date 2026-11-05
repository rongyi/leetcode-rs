#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        // Calculate time to reach the city for each monster
        let mut time_to_reach: Vec<f64> = dist
            .iter()
            .zip(speed.iter())
            .map(|(&d, &s)| d as f64 / s as f64)
            .collect();

        // Sort the times in ascending order
        time_to_reach.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Count how many monsters we can eliminate
        let mut count = 0;

        for (i, &time) in time_to_reach.iter().enumerate() {
            // If the monster arrives before we can shoot it, game over
            if time <= i as f64 {
                return count;
            }
            count += 1;
        }

        count
    }
}

fn main() {}
