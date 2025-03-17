#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn best_coordinate(towers: Vec<Vec<i32>>, radius: i32) -> Vec<i32> {
        let mut max_quality = 0;
        let mut best_coord = vec![0, 0];

        // Find the bounds for possible coordinates
        let mut max_x = 0;
        let mut max_y = 0;
        for tower in &towers {
            max_x = max_x.max(tower[0]);
            max_y = max_y.max(tower[1]);
        }

        // Check each possible coordinate
        for x in 0..=max_x {
            for y in 0..=max_y {
                let mut quality = 0;

                for tower in &towers {
                    let tower_x = tower[0];
                    let tower_y = tower[1];
                    let tower_q = tower[2];

                    // Calculate Euclidean distance
                    let distance = (((tower_x - x).pow(2) + (tower_y - y).pow(2)) as f64).sqrt();

                    // If within radius, add to quality
                    if distance <= radius as f64 {
                        quality += (tower_q as f64 / (1.0 + distance)).floor() as i32;
                    }
                }

                // Update best coordinates if better quality found
                // or same quality but lexicographically smaller
                if quality > max_quality
                    || (quality == max_quality
                        && (x < best_coord[0] || (x == best_coord[0] && y < best_coord[1])))
                {
                    max_quality = quality;
                    best_coord = vec![x, y];
                }
            }
        }

        best_coord
    }
}

fn main() {}
